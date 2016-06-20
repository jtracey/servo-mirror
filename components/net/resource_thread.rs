/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! A thread that takes a URL and streams back the binary data.
use about_loader;
use chrome_loader;
use connector::{Connector, create_http_connector};
use content_blocker::BLOCKED_CONTENT_RULES;
use cookie;
use cookie_storage::CookieStorage;
use data_loader;
use devtools_traits::DevtoolsControlMsg;
use fetch::methods::{fetch, FetchContext};
use file_loader;
use filemanager_thread::{FileManagerThreadFactory, TFDProvider};
use hsts::HstsList;
use http_loader::{self, HttpState};
use hyper::client::pool::Pool;
use hyper::header::{ContentType, Header, SetCookie};
use hyper::mime::{Mime, SubLevel, TopLevel};
use ipc_channel::ipc::{self, IpcReceiver, IpcSender, IpcReceiverSet};
use mime_classifier::{ApacheBugFlag, MIMEClassifier, NoSniffFlag};
use net_traits::LoadContext;
use net_traits::ProgressMsg::Done;
use net_traits::filemanager_thread::FileManagerThreadMsg;
use net_traits::request::{Request, RequestInit};
use net_traits::storage_thread::StorageThreadMsg;
use net_traits::{AsyncResponseTarget, Metadata, ProgressMsg, ResponseAction, CoreResourceThread};
use net_traits::{CoreResourceMsg, CookieSource, FetchResponseMsg, FetchTaskTarget, LoadConsumer};
use net_traits::{LoadData, LoadResponse, NetworkError, ResourceId};
use net_traits::{WebSocketCommunicate, WebSocketConnectData, ResourceThreads};
use profile_traits::time::ProfilerChan;
use rustc_serialize::json;
use rustc_serialize::{Decodable, Encodable};
use std::borrow::ToOwned;
use std::boxed::FnBox;
use std::cell::Cell;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, RwLock};
use storage_thread::StorageThreadFactory;
use url::Url;
use util::opts;
use util::prefs;
use util::thread::spawn_named;
use websocket_loader;

const TFD_PROVIDER: &'static TFDProvider = &TFDProvider;

pub enum ProgressSender {
    Channel(IpcSender<ProgressMsg>),
    Listener(AsyncResponseTarget),
}

#[derive(Clone)]
pub struct ResourceGroup {
    cookie_jar: Arc<RwLock<CookieStorage>>,
    auth_cache: Arc<RwLock<AuthCache>>,
    hsts_list: Arc<RwLock<HstsList>>,
    connector: Arc<Pool<Connector>>,
}

impl ProgressSender {
    //XXXjdm return actual error
    pub fn send(&self, msg: ProgressMsg) -> Result<(), ()> {
        match *self {
            ProgressSender::Channel(ref c) => c.send(msg).map_err(|_| ()),
            ProgressSender::Listener(ref b) => {
                let action = match msg {
                    ProgressMsg::Payload(buf) => ResponseAction::DataAvailable(buf),
                    ProgressMsg::Done(status) => ResponseAction::ResponseComplete(status),
                };
                b.invoke_with_listener(action);
                Ok(())
            }
        }
    }
}

pub fn send_error(url: Url, err: NetworkError, start_chan: LoadConsumer) {
    let mut metadata: Metadata = Metadata::default(url);
    metadata.status = None;

    if let Ok(p) = start_sending_opt(start_chan, metadata, Some(err.clone())) {
        p.send(Done(Err(err))).unwrap();
    }
}

/// For use by loaders in responding to a Load message that allows content sniffing.
pub fn start_sending_sniffed(start_chan: LoadConsumer, metadata: Metadata,
                             classifier: Arc<MIMEClassifier>, partial_body: &[u8],
                             context: LoadContext)
                             -> ProgressSender {
    start_sending_sniffed_opt(start_chan, metadata, classifier, partial_body, context).ok().unwrap()
}

/// For use by loaders in responding to a Load message that allows content sniffing.
pub fn start_sending_sniffed_opt(start_chan: LoadConsumer, mut metadata: Metadata,
                                 classifier: Arc<MIMEClassifier>, partial_body: &[u8],
                                 context: LoadContext)
                                 -> Result<ProgressSender, ()> {
    if prefs::get_pref("network.mime.sniff").as_boolean().unwrap_or(false) {
        // TODO: should be calculated in the resource loader, from pull requeset #4094
        let mut no_sniff = NoSniffFlag::OFF;
        let mut check_for_apache_bug = ApacheBugFlag::OFF;

        if let Some(ref headers) = metadata.headers {
            if let Some(ref content_type) = headers.get_raw("content-type").and_then(|c| c.last()) {
                check_for_apache_bug = ApacheBugFlag::from_content_type(content_type)
            }
            if let Some(ref raw_content_type_options) = headers.get_raw("X-content-type-options") {
                if raw_content_type_options.iter().any(|ref opt| *opt == b"nosniff") {
                    no_sniff = NoSniffFlag::ON
                }
            }
        }

        let supplied_type =
            metadata.content_type.as_ref().map(|&ContentType(Mime(ref toplevel, ref sublevel, _))| {
            (format!("{}", toplevel), format!("{}", sublevel))
        });
        let (toplevel, sublevel) = classifier.classify(context,
                                                       no_sniff,
                                                       check_for_apache_bug,
                                                       &supplied_type,
                                                       &partial_body);
        let mime_tp: TopLevel = toplevel.parse().unwrap();
        let mime_sb: SubLevel = sublevel.parse().unwrap();
        metadata.content_type = Some(ContentType(Mime(mime_tp, mime_sb, vec![])));
    }

    start_sending_opt(start_chan, metadata, None)
}

/// For use by loaders in responding to a Load message.
/// It takes an optional NetworkError, so that we can extract the SSL Validation errors
/// and take it to the HTML parser
fn start_sending_opt(start_chan: LoadConsumer, metadata: Metadata,
                     network_error: Option<NetworkError>) -> Result<ProgressSender, ()> {
    match start_chan {
        LoadConsumer::Channel(start_chan) => {
            let (progress_chan, progress_port) = ipc::channel().unwrap();
            let result = start_chan.send(LoadResponse {
                metadata: metadata,
                progress_port: progress_port,
            });
            match result {
                Ok(_) => Ok(ProgressSender::Channel(progress_chan)),
                Err(_) => Err(())
            }
        }
        LoadConsumer::Listener(target) => {
            match network_error {
                Some(NetworkError::SslValidation(url)) => {
                    let error = NetworkError::SslValidation(url);
                    target.invoke_with_listener(ResponseAction::HeadersAvailable(Err(error)));
                }
                _ => target.invoke_with_listener(ResponseAction::HeadersAvailable(Ok(metadata))),
            }
            Ok(ProgressSender::Listener(target))
        }
    }
}

/// Returns a tuple of (public, private) senders to the new threads.
pub fn new_resource_threads(user_agent: String,
                            devtools_chan: Option<Sender<DevtoolsControlMsg>>,
                            profiler_chan: ProfilerChan) -> (ResourceThreads, ResourceThreads) {
    let (public_core, private_core) = new_core_resource_thread(user_agent, devtools_chan, profiler_chan);
    let storage: IpcSender<StorageThreadMsg> = StorageThreadFactory::new();
    let filemanager: IpcSender<FileManagerThreadMsg> = FileManagerThreadFactory::new(TFD_PROVIDER);
    (ResourceThreads::new(public_core, storage.clone(), filemanager.clone()),
     ResourceThreads::new(private_core, storage, filemanager))
}


/// Create a CoreResourceThread
pub fn new_core_resource_thread(user_agent: String,
                                devtools_chan: Option<Sender<DevtoolsControlMsg>>,
                                profiler_chan: ProfilerChan)
                                -> (CoreResourceThread, CoreResourceThread) {
    let (public_setup_chan, public_setup_port) = ipc::channel().unwrap();
    let (private_setup_chan, private_setup_port) = ipc::channel().unwrap();
    let public_setup_chan_clone = public_setup_chan.clone();
    let private_setup_chan_clone = private_setup_chan.clone();
    spawn_named("ResourceManager".to_owned(), move || {
        let resource_manager = CoreResourceManager::new(
            user_agent, devtools_chan, profiler_chan
        );

        let mut channel_manager = ResourceChannelManager {
            resource_manager: resource_manager,
        };
        channel_manager.start(public_setup_chan_clone,
                              private_setup_chan_clone,
                              public_setup_port,
                              private_setup_port);
    });
    (public_setup_chan, private_setup_chan)
}

struct ResourceChannelManager {
    resource_manager: CoreResourceManager,
}

fn create_resource_groups() -> (ResourceGroup, ResourceGroup) {
    let mut hsts_list = HstsList::from_servo_preload();
    let mut auth_cache = AuthCache::new();
    let mut cookie_jar = CookieStorage::new();
    if let Some(ref config_dir) = opts::get().config_dir {
        read_json_from_file(&mut auth_cache, config_dir, "auth_cache.json");
        read_json_from_file(&mut hsts_list, config_dir, "hsts_list.json");
        read_json_from_file(&mut cookie_jar, config_dir, "cookie_jar.json");
    }
    let resource_group = ResourceGroup {
        cookie_jar: Arc::new(RwLock::new(cookie_jar)),
        auth_cache: Arc::new(RwLock::new(auth_cache)),
        hsts_list: Arc::new(RwLock::new(hsts_list.clone())),
        connector: create_http_connector(),
    };
    let private_resource_group = ResourceGroup {
        cookie_jar: Arc::new(RwLock::new(CookieStorage::new())),
        auth_cache: Arc::new(RwLock::new(AuthCache::new())),
        hsts_list: Arc::new(RwLock::new(HstsList::new())),
        connector: create_http_connector(),
    };
    (resource_group, private_resource_group)
}

impl ResourceChannelManager {
    #[allow(unsafe_code)]
    fn start(&mut self,
             public_control_sender: CoreResourceThread,
             private_control_sender: CoreResourceThread,
             public_receiver: IpcReceiver<CoreResourceMsg>,
             private_receiver: IpcReceiver<CoreResourceMsg>) {
        let (public_resource_group, private_resource_group) = create_resource_groups();

        let mut rx_set = IpcReceiverSet::new().unwrap();
        let private_id = rx_set.add(private_receiver).unwrap();
        let public_id = rx_set.add(public_receiver).unwrap();

        loop {
            for (id, data) in rx_set.select().unwrap().into_iter().map(|m| m.unwrap()) {
                let (group, sender) = if id == private_id {
                    (&private_resource_group, &private_control_sender)
                } else {
                    assert_eq!(id, public_id);
                    (&public_resource_group, &public_control_sender)
                };
                if let Ok(msg) = data.to() {
                    if !self.process_msg(msg, group, &sender) {
                        break;
                    }
                }
            }
        }
    }

    /// Returns false if the thread should exit.
    fn process_msg(&mut self,
                   msg: CoreResourceMsg,
                   group: &ResourceGroup,
                   control_sender: &CoreResourceThread) -> bool {
        match msg {
            CoreResourceMsg::Load(load_data, consumer, id_sender) =>
                self.resource_manager.load(load_data, consumer, id_sender, control_sender.clone(), group),
            CoreResourceMsg::Fetch(init, sender) =>
                self.resource_manager.fetch(init, sender, group),
            CoreResourceMsg::WebsocketConnect(connect, connect_data) =>
                self.resource_manager.websocket_connect(connect, connect_data, group),
            CoreResourceMsg::SetCookiesForUrl(request, cookie_list, source) =>
                self.resource_manager.set_cookies_for_url(request, cookie_list, source, group),
            CoreResourceMsg::GetCookiesForUrl(url, consumer, source) => {
                let mut cookie_jar = group.cookie_jar.write().unwrap();
                consumer.send(cookie_jar.cookies_for_url(&url, source)).unwrap();
            }
            CoreResourceMsg::Cancel(res_id) => {
                if let Some(cancel_sender) = self.resource_manager.cancel_load_map.get(&res_id) {
                    let _ = cancel_sender.send(());
                }
                self.resource_manager.cancel_load_map.remove(&res_id);
            }
            CoreResourceMsg::Synchronize(sender) => {
                let _ = sender.send(());
            }
            CoreResourceMsg::Exit(sender) => {
                if let Some(ref config_dir) = opts::get().config_dir {
                    match group.auth_cache.read() {
                        Ok(auth_cache) => write_json_to_file(&*auth_cache, config_dir, "auth_cache.json"),
                        Err(_) => warn!("Error writing auth cache to disk"),
                    }
                    match group.cookie_jar.read() {
                        Ok(jar) => write_json_to_file(&*jar, config_dir, "cookie_jar.json"),
                        Err(_) => warn!("Error writing cookie jar to disk"),
                    }
                    match group.hsts_list.read() {
                        Ok(hsts) => write_json_to_file(&*hsts, config_dir, "hsts_list.json"),
                        Err(_) => warn!("Error writing hsts list to disk"),
                    }
                }
                let _ = sender.send(());
                return false;
            }
        }
        true
    }
}

pub fn read_json_from_file<T: Decodable>(data: &mut T, config_dir: &str, filename: &str) {
    let path = Path::new(config_dir).join(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => {
            warn!("couldn't open {}: {}", display, Error::description(&why));
            return;
        },
        Ok(file) => file,
    };

    let mut string_buffer: String = String::new();
    match file.read_to_string(&mut string_buffer) {
        Err(why) => {
            panic!("couldn't read from {}: {}", display,
                                                Error::description(&why))
        },
        Ok(_) => println!("successfully read from {}", display),
    }

    match json::decode(&string_buffer) {
        Ok(decoded_buffer) => *data = decoded_buffer,
        Err(why) => warn!("Could not decode buffer{}", why),
    }
}

pub fn write_json_to_file<T: Encodable>(data: &T, config_dir: &str, filename: &str) {
    let json_encoded: String;
    match json::encode(&data) {
        Ok(d) => json_encoded = d,
        Err(_) => return,
    }
    let path = Path::new(config_dir).join(filename);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}",
                           display,
                           Error::description(&why)),
        Ok(file) => file,
    };

    match file.write_all(json_encoded.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display,
                                               Error::description(&why))
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

/// The optional resources required by the `CancellationListener`
pub struct CancellableResource {
    /// The receiver which receives a message on load cancellation
    cancel_receiver: Receiver<()>,
    /// The `CancellationListener` is unique to this `ResourceId`
    resource_id: ResourceId,
    /// If we haven't initiated any cancel requests, then the loaders ask
    /// the listener to remove the `ResourceId` in the `HashMap` of
    /// `CoreResourceManager` once they finish loading
    resource_thread: CoreResourceThread,
}

impl CancellableResource {
    pub fn new(receiver: Receiver<()>, res_id: ResourceId, res_thread: CoreResourceThread) -> CancellableResource {
        CancellableResource {
            cancel_receiver: receiver,
            resource_id: res_id,
            resource_thread: res_thread,
        }
    }
}

/// A listener which is basically a wrapped optional receiver which looks
/// for the load cancellation message. Some of the loading processes always keep
/// an eye out for this message and stop loading stuff once they receive it.
pub struct CancellationListener {
    /// We'll be needing the resources only if we plan to cancel it
    cancel_resource: Option<CancellableResource>,
    /// This lets us know whether the request has already been cancelled
    cancel_status: Cell<bool>,
}

impl CancellationListener {
    pub fn new(resources: Option<CancellableResource>) -> CancellationListener {
        CancellationListener {
            cancel_resource: resources,
            cancel_status: Cell::new(false),
        }
    }

    pub fn is_cancelled(&self) -> bool {
        let resource = match self.cancel_resource {
            Some(ref resource) => resource,
            None => return false,  // channel doesn't exist!
        };
        if resource.cancel_receiver.try_recv().is_ok() {
            self.cancel_status.set(true);
            true
        } else {
            self.cancel_status.get()
        }
    }
}

impl Drop for CancellationListener {
    fn drop(&mut self) {
        if let Some(ref resource) = self.cancel_resource {
            // Ensure that the resource manager stops tracking this request now that it's terminated.
            let _ = resource.resource_thread.send(CoreResourceMsg::Cancel(resource.resource_id));
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct AuthCacheEntry {
    pub user_name: String,
    pub password: String,
}

impl AuthCache {
    pub fn new() -> AuthCache {
        AuthCache {
            version: 1,
            entries: HashMap::new()
        }
    }
}

#[derive(RustcDecodable, RustcEncodable, Clone)]
pub struct AuthCache {
    pub version: u32,
    pub entries: HashMap<Url, AuthCacheEntry>,
}

pub struct CoreResourceManager {
    user_agent: String,
    mime_classifier: Arc<MIMEClassifier>,
    devtools_chan: Option<Sender<DevtoolsControlMsg>>,
    profiler_chan: ProfilerChan,
    cancel_load_map: HashMap<ResourceId, Sender<()>>,
    next_resource_id: ResourceId,
}

impl CoreResourceManager {
    pub fn new(user_agent: String,
               devtools_channel: Option<Sender<DevtoolsControlMsg>>,
               profiler_chan: ProfilerChan) -> CoreResourceManager {
        CoreResourceManager {
            user_agent: user_agent,
            mime_classifier: Arc::new(MIMEClassifier::new()),
            devtools_chan: devtools_channel,
            profiler_chan: profiler_chan,
            cancel_load_map: HashMap::new(),
            next_resource_id: ResourceId(0),
        }
    }

    fn set_cookies_for_url(&mut self,
                           request: Url,
                           cookie_list: String,
                           source: CookieSource,
                           resource_group: &ResourceGroup) {
        let header = Header::parse_header(&[cookie_list.into_bytes()]);
        if let Ok(SetCookie(cookies)) = header {
            for bare_cookie in cookies {
                if let Some(cookie) = cookie::Cookie::new_wrapped(bare_cookie, &request, source) {
                    let mut cookie_jar = resource_group.cookie_jar.write().unwrap();
                    cookie_jar.push(cookie, source);
                }
            }
        }
    }

    fn load(&mut self,
            load_data: LoadData,
            consumer: LoadConsumer,
            id_sender: Option<IpcSender<ResourceId>>,
            resource_thread: CoreResourceThread,
            resource_grp: &ResourceGroup) {
        fn from_factory(factory: fn(LoadData, LoadConsumer, Arc<MIMEClassifier>, CancellationListener))
                        -> Box<FnBox(LoadData,
                                     LoadConsumer,
                                     Arc<MIMEClassifier>,
                                     CancellationListener) + Send> {
            box move |load_data, senders, classifier, cancel_listener| {
                factory(load_data, senders, classifier, cancel_listener)
            }
        }

        let cancel_resource = id_sender.map(|sender| {
            let current_res_id = self.next_resource_id;
            let _ = sender.send(current_res_id);
            let (cancel_sender, cancel_receiver) = channel();
            self.cancel_load_map.insert(current_res_id, cancel_sender);
            self.next_resource_id.0 += 1;
            CancellableResource::new(cancel_receiver, current_res_id, resource_thread)
        });

        let cancel_listener = CancellationListener::new(cancel_resource);
        let loader = match load_data.url.scheme() {
            "chrome" => from_factory(chrome_loader::factory),
            "file" => from_factory(file_loader::factory),
            "http" | "https" | "view-source" => {
                let http_state = HttpState {
                    blocked_content: BLOCKED_CONTENT_RULES.clone(),
                    hsts_list: resource_grp.hsts_list.clone(),
                    cookie_jar: resource_grp.cookie_jar.clone(),
                    auth_cache: resource_grp.auth_cache.clone()
                };
                http_loader::factory(self.user_agent.clone(),
                                     http_state,
                                     self.devtools_chan.clone(),
                                     self.profiler_chan.clone(),
                                     resource_grp.connector.clone())
            },
            "data" => from_factory(data_loader::factory),
            "about" => from_factory(about_loader::factory),
            _ => {
                debug!("resource_thread: no loader for scheme {}", load_data.url.scheme());
                send_error(load_data.url, NetworkError::Internal("no loader for scheme".to_owned()), consumer);
                return
            }
        };
        debug!("resource_thread: loading url: {}", load_data.url);

        loader.call_box((load_data,
                         consumer,
                         self.mime_classifier.clone(),
                         cancel_listener));
    }

    fn fetch(&self,
             init: RequestInit,
             sender: IpcSender<FetchResponseMsg>,
             group: &ResourceGroup) {
        let http_state = HttpState {
            hsts_list: group.hsts_list.clone(),
            cookie_jar: group.cookie_jar.clone(),
            auth_cache: group.auth_cache.clone(),
            blocked_content: BLOCKED_CONTENT_RULES.clone(),
        };
        let ua = self.user_agent.clone();
        spawn_named(format!("fetch thread for {}", init.url), move || {
            let request = Request::from_init(init);
            // XXXManishearth: Check origin against pipeline id (also ensure that the mode is allowed)
            // todo load context / mimesniff in fetch
            // todo referrer policy?
            // todo service worker stuff
            let mut target = Some(Box::new(sender) as Box<FetchTaskTarget + Send + 'static>);
            let context = FetchContext { state: http_state, user_agent: ua };
            fetch(Rc::new(request), &mut target, context);
        })
    }

    fn websocket_connect(&self,
                         connect: WebSocketCommunicate,
                         connect_data: WebSocketConnectData,
                         resource_grp: &ResourceGroup) {
        websocket_loader::init(connect, connect_data, resource_grp.cookie_jar.clone());
    }
}

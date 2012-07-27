/*!

A task that takes a URL and streams back the binary data

*/

export ControlMsg, ProgressMsg, ResourceTask, ResourceManager, LoaderTaskFactory;

import comm::{chan, port, methods};
import task::{spawn, spawn_listener};
import std::net::url;
import std::net::url::url;
import result::{result, ok, err};

enum ControlMsg {
    Load(url, chan<ProgressMsg>),
    Exit
}

enum ProgressMsg {
    Payload(~[u8]),
    Done(result<(), ()>)
}

type ResourceTask = chan<ControlMsg>;
/// Creates a task to load a specific resource
type LoaderTaskFactory = fn~(url: url, chan<ProgressMsg>);

fn ResourceTask() -> ResourceTask {
    create_resource_task_with_loaders(~[])
}

fn create_resource_task_with_loaders(+loaders: ~[(~str, LoaderTaskFactory)]) -> ResourceTask {
    do spawn_listener |from_client| {
        ResourceManager(from_client, loaders).start()
    }
}

class ResourceManager {
    let from_client: port<ControlMsg>;
    /// Per-scheme resource loaders
    let loaders: ~[(~str, LoaderTaskFactory)];

    new(from_client: port<ControlMsg>, loaders: ~[(~str, LoaderTaskFactory)]) {
        self.from_client = from_client;
        self.loaders = loaders;
    }

    fn start() {
        loop {
            alt self.from_client.recv() {
              Load(url, progress_chan) {
                self.load(url, progress_chan)
              }
              Exit {
                break
              }
            }
        }
    }

    fn load(url: url, progress_chan: chan<ProgressMsg>) {

        alt self.get_loader_factory(url) {
          some(loader_factory) {
            loader_factory(url, progress_chan);
          }
          none {
            #debug("resource_task: no loader for scheme %s", url.scheme);
            progress_chan.send(Done(err(())));
          }
        }
    }

    fn get_loader_factory(url: url) -> option<LoaderTaskFactory> {
        for self.loaders.each |scheme_loader| {
            let (scheme, loader_factory) = scheme_loader;
            if scheme == url.scheme {
                ret some(loader_factory);
            }
        }
        ret none;
    }
}

#[test]
fn test_exit() {
    let resource_task = ResourceTask();
    resource_task.send(Exit);
}

#[test]
fn test_bad_scheme() {
    let resource_task = ResourceTask();
    let progress = port();
    resource_task.send(Load(url::from_str(~"bogus://whatever").get(), progress.chan()));
    alt check progress.recv() {
      Done(result) { assert result.is_err() }
    }
    resource_task.send(Exit);
}

#[test]
fn should_delegate_to_scheme_loader() {
    let payload = ~[1, 2, 3];
    let loader_factory = fn~(url: url, progress_chan: chan<ProgressMsg>) {
        progress_chan.send(Payload(payload));
        progress_chan.send(Done(ok(())));
    };
    let loader_factories = ~[(~"snicklefritz", loader_factory)];
    let resource_task = create_resource_task_with_loaders(loader_factories);
    let progress = port();
    resource_task.send(Load(url::from_str(~"snicklefritz://heya").get(), progress.chan()));
    assert progress.recv() == Payload(payload);
    assert progress.recv() == Done(ok(()));
    resource_task.send(Exit);
}
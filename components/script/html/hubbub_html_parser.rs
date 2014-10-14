/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::attr::AttrHelpers;
use dom::bindings::codegen::Bindings::AttrBinding::AttrMethods;
use dom::bindings::codegen::Bindings::NodeBinding::NodeMethods;
use dom::bindings::codegen::InheritTypes::{NodeBase, NodeCast, TextCast};
use dom::bindings::codegen::InheritTypes::{ElementCast, HTMLScriptElementCast};
use dom::bindings::js::{JS, JSRef, Temporary, OptionalRootable, Root};
use dom::bindings::utils::Reflectable;
use dom::document::{Document, DocumentHelpers};
use dom::element::AttributeHandlers;
use dom::htmlelement::HTMLElement;
use dom::htmlheadingelement::{Heading1, Heading2, Heading3, Heading4, Heading5, Heading6};
use dom::htmlformelement::HTMLFormElement;
use dom::htmlscriptelement::HTMLScriptElementHelpers;
use dom::node::NodeHelpers;
use dom::types::*;
use page::Page;

use encoding::all::UTF_8;
use encoding::types::{Encoding, DecodeReplace};

use hubbub::hubbub;
use hubbub::hubbub::{NullNs, HtmlNs, MathMlNs, SvgNs, XLinkNs, XmlNs, XmlNsNs};
use servo_net::resource_task::{Load, LoadData, Payload, Done, ResourceTask, load_whole_resource};
use servo_msg::constellation_msg::LoadData as MsgLoadData;
use servo_util::str::DOMString;
use servo_util::task::spawn_named;
use std::ascii::StrAsciiExt;
use std::mem;
use std::cell::RefCell;
use std::comm::{channel, Sender, Receiver};
use url::{Url, UrlParser};
use http::headers::HeaderEnum;
use time;
use string_cache::{Atom, Namespace};

macro_rules! handle_element(
    ($document: expr,
     $localName: expr,
     $prefix: expr,
     $string: expr,
     $ctor: ident
     $(, $arg:expr )*) => (
        if $string == $localName.as_slice() {
            return ElementCast::from_temporary($ctor::new($localName, $prefix, $document $(, $arg)*));
        }
    )
)


pub struct JSFile {
    pub data: String,
    pub url: Option<Url>,
}

pub type JSResult = Vec<JSFile>;

pub enum HTMLInput {
    InputString(String),
    InputUrl(Url),
}

enum JSMessage {
    JSTaskNewFile(Url),
    JSTaskNewInlineScript(String, Option<Url>),
    JSTaskExit
}

/// Messages generated by the HTML parser upon discovery of additional resources
pub enum HtmlDiscoveryMessage {
    HtmlDiscoveredScript(JSResult)
}

pub struct HtmlParserResult {
    pub discovery_port: Receiver<HtmlDiscoveryMessage>,
}

trait NodeWrapping<T> {
    unsafe fn to_hubbub_node(&self) -> hubbub::NodeDataPtr;
}

impl<'a, T: NodeBase+Reflectable> NodeWrapping<T> for JSRef<'a, T> {
    unsafe fn to_hubbub_node(&self) -> hubbub::NodeDataPtr {
        mem::transmute(self.deref())
    }
}

unsafe fn from_hubbub_node<T: Reflectable>(n: hubbub::NodeDataPtr) -> Temporary<T> {
    Temporary::new(JS::from_raw(mem::transmute(n)))
}

fn js_script_listener(to_parent: Sender<HtmlDiscoveryMessage>,
                      from_parent: Receiver<JSMessage>,
                      resource_task: ResourceTask) {
    let mut result_vec = vec!();

    loop {
        match from_parent.recv_opt() {
            Ok(JSTaskNewFile(url)) => {
                match load_whole_resource(&resource_task, url.clone()) {
                    Err(_) => {
                        error!("error loading script {:s}", url.serialize());
                    }
                    Ok((metadata, bytes)) => {
                        let decoded = UTF_8.decode(bytes.as_slice(), DecodeReplace).unwrap();
                        result_vec.push(JSFile {
                            data: decoded.to_string(),
                            url: Some(metadata.final_url),
                        });
                    }
                }
            }
            Ok(JSTaskNewInlineScript(data, url)) => {
                result_vec.push(JSFile { data: data, url: url });
            }
            Ok(JSTaskExit) | Err(()) => {
                break;
            }
        }
    }

    assert!(to_parent.send_opt(HtmlDiscoveredScript(result_vec)).is_ok());
}

// Parses an RFC 2616 compliant date/time string, and returns a localized
// date/time string in a format suitable for document.lastModified.
fn parse_last_modified(timestamp: &str) -> String {
    let format = "%m/%d/%Y %H:%M:%S";

    // RFC 822, updated by RFC 1123
    match time::strptime(timestamp, "%a, %d %b %Y %T %Z") {
        Ok(t) => return t.to_local().strftime(format),
        Err(_) => ()
    }

    // RFC 850, obsoleted by RFC 1036
    match time::strptime(timestamp, "%A, %d-%b-%y %T %Z") {
        Ok(t) => return t.to_local().strftime(format),
        Err(_) => ()
    }

    // ANSI C's asctime() format
    match time::strptime(timestamp, "%c") {
        Ok(t) => t.to_local().strftime(format),
        Err(_) => String::from_str("")
    }
}

// Silly macros to handle constructing      DOM nodes. This produces bad code and should be optimized
// via atomization (issue #85).

pub fn build_element_from_tag(tag: DOMString, ns: Namespace, prefix: Option<DOMString>, document: JSRef<Document>) -> Temporary<Element> {
    if ns != ns!(HTML) {
        return Element::new(tag, ns, prefix, document);
    }

    // TODO (Issue #85): use atoms
    handle_element!(document, tag, prefix, "a",         HTMLAnchorElement);
    handle_element!(document, tag, prefix, "abbr",      HTMLElement);
    handle_element!(document, tag, prefix, "acronym",   HTMLElement);
    handle_element!(document, tag, prefix, "address",   HTMLElement);
    handle_element!(document, tag, prefix, "applet",    HTMLAppletElement);
    handle_element!(document, tag, prefix, "area",      HTMLAreaElement);
    handle_element!(document, tag, prefix, "article",   HTMLElement);
    handle_element!(document, tag, prefix, "aside",     HTMLElement);
    handle_element!(document, tag, prefix, "audio",     HTMLAudioElement);
    handle_element!(document, tag, prefix, "b",         HTMLElement);
    handle_element!(document, tag, prefix, "base",      HTMLBaseElement);
    handle_element!(document, tag, prefix, "bdi",       HTMLElement);
    handle_element!(document, tag, prefix, "bdo",       HTMLElement);
    handle_element!(document, tag, prefix, "bgsound",   HTMLElement);
    handle_element!(document, tag, prefix, "big",       HTMLElement);
    handle_element!(document, tag, prefix, "blockquote",HTMLElement);
    handle_element!(document, tag, prefix, "body",      HTMLBodyElement);
    handle_element!(document, tag, prefix, "br",        HTMLBRElement);
    handle_element!(document, tag, prefix, "button",    HTMLButtonElement);
    handle_element!(document, tag, prefix, "canvas",    HTMLCanvasElement);
    handle_element!(document, tag, prefix, "caption",   HTMLTableCaptionElement);
    handle_element!(document, tag, prefix, "center",    HTMLElement);
    handle_element!(document, tag, prefix, "cite",      HTMLElement);
    handle_element!(document, tag, prefix, "code",      HTMLElement);
    handle_element!(document, tag, prefix, "col",       HTMLTableColElement);
    handle_element!(document, tag, prefix, "colgroup",  HTMLTableColElement);
    handle_element!(document, tag, prefix, "data",      HTMLDataElement);
    handle_element!(document, tag, prefix, "datalist",  HTMLDataListElement);
    handle_element!(document, tag, prefix, "dd",        HTMLElement);
    handle_element!(document, tag, prefix, "del",       HTMLModElement);
    handle_element!(document, tag, prefix, "details",   HTMLElement);
    handle_element!(document, tag, prefix, "dfn",       HTMLElement);
    handle_element!(document, tag, prefix, "dir",       HTMLDirectoryElement);
    handle_element!(document, tag, prefix, "div",       HTMLDivElement);
    handle_element!(document, tag, prefix, "dl",        HTMLDListElement);
    handle_element!(document, tag, prefix, "dt",        HTMLElement);
    handle_element!(document, tag, prefix, "em",        HTMLElement);
    handle_element!(document, tag, prefix, "embed",     HTMLEmbedElement);
    handle_element!(document, tag, prefix, "fieldset",  HTMLFieldSetElement);
    handle_element!(document, tag, prefix, "figcaption",HTMLElement);
    handle_element!(document, tag, prefix, "figure",    HTMLElement);
    handle_element!(document, tag, prefix, "font",      HTMLFontElement);
    handle_element!(document, tag, prefix, "footer",    HTMLElement);
    handle_element!(document, tag, prefix, "form",      HTMLFormElement);
    handle_element!(document, tag, prefix, "frame",     HTMLFrameElement);
    handle_element!(document, tag, prefix, "frameset",  HTMLFrameSetElement);
    handle_element!(document, tag, prefix, "h1",        HTMLHeadingElement, Heading1);
    handle_element!(document, tag, prefix, "h2",        HTMLHeadingElement, Heading2);
    handle_element!(document, tag, prefix, "h3",        HTMLHeadingElement, Heading3);
    handle_element!(document, tag, prefix, "h4",        HTMLHeadingElement, Heading4);
    handle_element!(document, tag, prefix, "h5",        HTMLHeadingElement, Heading5);
    handle_element!(document, tag, prefix, "h6",        HTMLHeadingElement, Heading6);
    handle_element!(document, tag, prefix, "head",      HTMLHeadElement);
    handle_element!(document, tag, prefix, "header",    HTMLElement);
    handle_element!(document, tag, prefix, "hgroup",    HTMLElement);
    handle_element!(document, tag, prefix, "hr",        HTMLHRElement);
    handle_element!(document, tag, prefix, "html",      HTMLHtmlElement);
    handle_element!(document, tag, prefix, "i",         HTMLElement);
    handle_element!(document, tag, prefix, "iframe",    HTMLIFrameElement);
    handle_element!(document, tag, prefix, "img",       HTMLImageElement);
    handle_element!(document, tag, prefix, "input",     HTMLInputElement);
    handle_element!(document, tag, prefix, "ins",       HTMLModElement);
    handle_element!(document, tag, prefix, "isindex",   HTMLElement);
    handle_element!(document, tag, prefix, "kbd",       HTMLElement);
    handle_element!(document, tag, prefix, "label",     HTMLLabelElement);
    handle_element!(document, tag, prefix, "legend",    HTMLLegendElement);
    handle_element!(document, tag, prefix, "li",        HTMLLIElement);
    handle_element!(document, tag, prefix, "link",      HTMLLinkElement);
    handle_element!(document, tag, prefix, "main",      HTMLElement);
    handle_element!(document, tag, prefix, "map",       HTMLMapElement);
    handle_element!(document, tag, prefix, "mark",      HTMLElement);
    handle_element!(document, tag, prefix, "marquee",   HTMLElement);
    handle_element!(document, tag, prefix, "meta",      HTMLMetaElement);
    handle_element!(document, tag, prefix, "meter",     HTMLMeterElement);
    handle_element!(document, tag, prefix, "nav",       HTMLElement);
    handle_element!(document, tag, prefix, "nobr",      HTMLElement);
    handle_element!(document, tag, prefix, "noframes",  HTMLElement);
    handle_element!(document, tag, prefix, "noscript",  HTMLElement);
    handle_element!(document, tag, prefix, "object",    HTMLObjectElement);
    handle_element!(document, tag, prefix, "ol",        HTMLOListElement);
    handle_element!(document, tag, prefix, "optgroup",  HTMLOptGroupElement);
    handle_element!(document, tag, prefix, "option",    HTMLOptionElement);
    handle_element!(document, tag, prefix, "output",    HTMLOutputElement);
    handle_element!(document, tag, prefix, "p",         HTMLParagraphElement);
    handle_element!(document, tag, prefix, "param",     HTMLParamElement);
    handle_element!(document, tag, prefix, "pre",       HTMLPreElement);
    handle_element!(document, tag, prefix, "progress",  HTMLProgressElement);
    handle_element!(document, tag, prefix, "q",         HTMLQuoteElement);
    handle_element!(document, tag, prefix, "rp",        HTMLElement);
    handle_element!(document, tag, prefix, "rt",        HTMLElement);
    handle_element!(document, tag, prefix, "ruby",      HTMLElement);
    handle_element!(document, tag, prefix, "s",         HTMLElement);
    handle_element!(document, tag, prefix, "samp",      HTMLElement);
    handle_element!(document, tag, prefix, "script",    HTMLScriptElement);
    handle_element!(document, tag, prefix, "section",   HTMLElement);
    handle_element!(document, tag, prefix, "select",    HTMLSelectElement);
    handle_element!(document, tag, prefix, "small",     HTMLElement);
    handle_element!(document, tag, prefix, "source",    HTMLSourceElement);
    handle_element!(document, tag, prefix, "spacer",    HTMLElement);
    handle_element!(document, tag, prefix, "span",      HTMLSpanElement);
    handle_element!(document, tag, prefix, "strike",    HTMLElement);
    handle_element!(document, tag, prefix, "strong",    HTMLElement);
    handle_element!(document, tag, prefix, "style",     HTMLStyleElement);
    handle_element!(document, tag, prefix, "sub",       HTMLElement);
    handle_element!(document, tag, prefix, "summary",   HTMLElement);
    handle_element!(document, tag, prefix, "sup",       HTMLElement);
    handle_element!(document, tag, prefix, "table",     HTMLTableElement);
    handle_element!(document, tag, prefix, "tbody",     HTMLTableSectionElement);
    handle_element!(document, tag, prefix, "td",        HTMLTableDataCellElement);
    handle_element!(document, tag, prefix, "template",  HTMLTemplateElement);
    handle_element!(document, tag, prefix, "textarea",  HTMLTextAreaElement);
    handle_element!(document, tag, prefix, "th",        HTMLTableHeaderCellElement);
    handle_element!(document, tag, prefix, "time",      HTMLTimeElement);
    handle_element!(document, tag, prefix, "title",     HTMLTitleElement);
    handle_element!(document, tag, prefix, "tr",        HTMLTableRowElement);
    handle_element!(document, tag, prefix, "tt",        HTMLElement);
    handle_element!(document, tag, prefix, "track",     HTMLTrackElement);
    handle_element!(document, tag, prefix, "u",         HTMLElement);
    handle_element!(document, tag, prefix, "ul",        HTMLUListElement);
    handle_element!(document, tag, prefix, "var",       HTMLElement);
    handle_element!(document, tag, prefix, "video",     HTMLVideoElement);
    handle_element!(document, tag, prefix, "wbr",       HTMLElement);

    return ElementCast::from_temporary(HTMLUnknownElement::new(tag, prefix, document));
}

// The url from msg_load_data is ignored here
pub fn parse_html(page: &Page,
                  document: JSRef<Document>,
                  input: HTMLInput,
                  resource_task: ResourceTask,
                  msg_load_data: Option<MsgLoadData>)
                  -> HtmlParserResult {
    debug!("Hubbub: parsing {:?}", input);

    // Spawn a JS parser to receive JavaScript.
    let (discovery_chan, discovery_port) = channel();
    let resource_task2 = resource_task.clone();
    let js_result_chan = discovery_chan.clone();
    let (js_chan, js_msg_port) = channel();
    spawn_named("parse_html:js", proc() {
        js_script_listener(js_result_chan, js_msg_port, resource_task2.clone());
    });

    let (base_url, load_response) = match input {
        InputUrl(ref url) => {
            // Wait for the LoadResponse so that the parser knows the final URL.
            let (input_chan, input_port) = channel();
            let mut load_data = LoadData::new(url.clone());
            msg_load_data.map(|m| {
                load_data.headers = m.headers;
                load_data.method = m.method;
                load_data.data = m.data;
            });
            resource_task.send(Load(load_data, input_chan));

            let load_response = input_port.recv();

            debug!("Fetched page; metadata is {:?}", load_response.metadata);

            load_response.metadata.headers.as_ref().map(|headers| {
                let header = headers.iter().find(|h|
                    h.header_name().as_slice().to_ascii_lower() == "last-modified".to_string()
                );

                match header {
                    Some(h) => document.set_last_modified(
                        parse_last_modified(h.header_value().as_slice())),
                    None => {},
                };
            });

            let base_url = load_response.metadata.final_url.clone();

            {
                // Store the final URL before we start parsing, so that DOM routines
                // (e.g. HTMLImageElement::update_image) can resolve relative URLs
                // correctly.
                *page.mut_url() = Some((base_url.clone(), true));
            }

            (Some(base_url), Some(load_response))
        },
        InputString(_) => {
            match *page.url() {
                Some((ref page_url, _)) => (Some(page_url.clone()), None),
                None => (None, None),
            }
        },
    };

    let mut parser = build_parser(unsafe { document.to_hubbub_node() });
    debug!("created parser");

    let js_chan2 = js_chan.clone();

    let doc_cell = RefCell::new(document);

    let mut tree_handler = hubbub::TreeHandler {
        create_comment: |data: String| {
            debug!("create comment");
            // NOTE: tmp vars are workaround for lifetime issues. Both required.
            let tmp_borrow = doc_cell.borrow();
            let tmp = &*tmp_borrow;
            let comment = Comment::new(data, *tmp).root();
            let comment: JSRef<Node> = NodeCast::from_ref(*comment);
            unsafe { comment.to_hubbub_node() }
        },
        create_doctype: |box hubbub::Doctype { name: name, public_id: public_id, system_id: system_id, ..}: Box<hubbub::Doctype>| {
            debug!("create doctype");
            // NOTE: tmp vars are workaround for lifetime issues. Both required.
            let tmp_borrow = doc_cell.borrow();
            let tmp = &*tmp_borrow;
            let doctype_node = DocumentType::new(name, public_id, system_id, *tmp).root();
            unsafe {
                doctype_node.to_hubbub_node()
            }
        },
        create_element: |tag: Box<hubbub::Tag>| {
            debug!("create element {}", tag.name);
            // NOTE: tmp vars are workaround for lifetime issues. Both required.
            let tmp_borrow = doc_cell.borrow();
            let tmp = &*tmp_borrow;
            let namespace = match tag.ns {
                HtmlNs => ns!(HTML),
                MathMlNs => ns!(MathML),
                SvgNs => ns!(SVG),
                ns => fail!("Not expecting namespace {:?}", ns),
            };
            let element: Root<Element> = build_element_from_tag(tag.name.clone(), namespace, None, *tmp).root();

            debug!("-- attach attrs");
            for attr in tag.attributes.iter() {
                let (namespace, prefix) = match attr.ns {
                    NullNs => (ns!(""), None),
                    XLinkNs => (ns!(XLink), Some("xlink")),
                    XmlNs => (ns!(XML), Some("xml")),
                    XmlNsNs => (ns!(XMLNS), Some("xmlns")),
                    ns => fail!("Not expecting namespace {:?}", ns),
                };
                element.set_attribute_from_parser(Atom::from_slice(attr.name.as_slice()),
                                                  attr.value.clone(),
                                                  namespace,
                                                  prefix.map(|p| p.to_string()));
            }

            unsafe { element.to_hubbub_node() }
        },
        create_text: |data: String| {
            debug!("create text");
            // NOTE: tmp vars are workaround for lifetime issues. Both required.
            let tmp_borrow = doc_cell.borrow();
            let tmp = &*tmp_borrow;
            let text = Text::new(data, *tmp).root();
            unsafe { text.to_hubbub_node() }
        },
        ref_node: |_| {},
        unref_node: |_| {},
        append_child: |parent: hubbub::NodeDataPtr, child: hubbub::NodeDataPtr| {
            unsafe {
                debug!("append child {:x} {:x}", parent, child);
                let child: Root<Node> = from_hubbub_node(child).root();
                let parent: Root<Node> = from_hubbub_node(parent).root();
                assert!(parent.AppendChild(*child).is_ok());
            }
            child
        },
        insert_before: |_parent, _child| {
            debug!("insert before");
            0u
        },
        remove_child: |_parent, _child| {
            debug!("remove child");
            0u
        },
        clone_node: |_node, deep| {
            debug!("clone node");
            if deep { error!("-- deep clone unimplemented"); }
            fail!("clone node unimplemented")
        },
        reparent_children: |_node, _new_parent| {
            debug!("reparent children");
            0u
        },
        get_parent: |_node, _element_only| {
            debug!("get parent");
            0u
        },
        has_children: |_node| {
            debug!("has children");
            false
        },
        form_associate: |_form, _node| {
            debug!("form associate");
        },
        add_attributes: |_node, _attributes| {
            debug!("add attributes");
        },
        set_quirks_mode: |mode| {
            debug!("set quirks mode");
            // NOTE: tmp vars are workaround for lifetime issues. Both required.
            let tmp_borrow = doc_cell.borrow_mut();
            let tmp = &*tmp_borrow;
            tmp.set_quirks_mode(mode);
        },
        encoding_change: |encname| {
            debug!("encoding change");
            // NOTE: tmp vars are workaround for lifetime issues. Both required.
            let tmp_borrow = doc_cell.borrow_mut();
            let tmp = &*tmp_borrow;
            tmp.set_encoding_name(encname);
        },
        complete_script: |script| {
            unsafe {
                let script = from_hubbub_node::<Node>(script).root();
                let script: Option<JSRef<HTMLScriptElement>> =
                    HTMLScriptElementCast::to_ref(*script);
                let script = match script {
                    Some(script) if script.is_javascript() => script,
                    _ => return,
                };

                let script_element: JSRef<Element> = ElementCast::from_ref(script);
                match script_element.get_attribute(ns!(""), "src").root() {
                    Some(src) => {
                        debug!("found script: {:s}", src.Value());
                        let mut url_parser = UrlParser::new();
                        match base_url {
                            None => (),
                            Some(ref base_url) => {
                                url_parser.base_url(base_url);
                            }
                        };
                        match url_parser.parse(src.value().as_slice()) {
                            Ok(new_url) => js_chan2.send(JSTaskNewFile(new_url)),
                            Err(e) => debug!("Parsing url {:s} failed: {:?}", src.Value(), e)
                        };
                    }
                    None => {
                        let mut data = String::new();
                        let scriptnode: JSRef<Node> = NodeCast::from_ref(script);
                        debug!("iterating over children {:?}", scriptnode.first_child());
                        for child in scriptnode.children() {
                            debug!("child = {:?}", child);
                            let text: JSRef<Text> = TextCast::to_ref(child).unwrap();
                            data.push_str(text.characterdata().data().as_slice());
                        }

                        debug!("script data = {:?}", data);
                        js_chan2.send(JSTaskNewInlineScript(data, base_url.clone()));
                    }
                }
            }
            debug!("complete script");
        },
        complete_style: |_| {
            // style parsing is handled in element::notify_child_list_changed.
        },
    };
    parser.set_tree_handler(&mut tree_handler);
    debug!("set tree handler");
    debug!("loaded page");
    match input {
        InputString(s) => {
            parser.parse_chunk(s.into_bytes().as_slice());
        },
        InputUrl(url) => {
            let load_response = load_response.unwrap();
            match load_response.metadata.content_type {
                Some((ref t, _)) if t.as_slice().eq_ignore_ascii_case("image") => {
                    let page = format!("<html><body><img src='{:s}' /></body></html>", base_url.as_ref().unwrap().serialize());
                    parser.parse_chunk(page.into_bytes().as_slice());
                },
                _ => loop {
                    match load_response.progress_port.recv() {
                        Payload(data) => {
                            debug!("received data");
                            parser.parse_chunk(data.as_slice());
                        }
                        Done(Err(err)) => {
                            fail!("Failed to load page URL {:s}, error: {:s}", url.serialize(), err);
                        }
                        Done(..) => {
                            break;
                        }
                    }
                }
            }
        },
    }

    debug!("finished parsing");
    js_chan.send(JSTaskExit);

    HtmlParserResult {
        discovery_port: discovery_port,
    }
}

fn build_parser<'a>(node: hubbub::NodeDataPtr) -> hubbub::Parser<'a> {
    let mut parser = hubbub::Parser::new("UTF-8", false);
    parser.set_document_node(node);
    parser.enable_scripting(true);
    parser.enable_styling(true);
    parser
}


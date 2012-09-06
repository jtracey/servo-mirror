#[doc="The core DOM types. Defines the basic DOM hierarchy as well as all the HTML elements."]

import gfx::geometry::au;
import geom::size::Size2D;
import layout::base::LayoutData;
import util::tree;
import js::rust::{bare_compartment, compartment, methods};
import js::jsapi::{JSClass, JSObject, JSPropertySpec, JSContext, jsid, jsval, JSBool};
import js::{JSPROP_ENUMERATE, JSPROP_SHARED};
import js::crust::*;
import js::glue::bindgen::RUST_OBJECT_TO_JSVAL;
import dvec::DVec;
import ptr::null;
import bindings;
import std::arc::ARC;
import style::Stylesheet;
import comm::{Port, Chan};
import content::content_task::{ControlMsg, Timer};

enum TimerControlMsg {
    Fire(~dom::bindings::window::TimerData),
    Close
}

struct Window {
    let timer_chan: Chan<TimerControlMsg>;

    new(content_port: Port<ControlMsg>) {
        let content_chan = Chan(content_port);
        
        self.timer_chan = do task::spawn_listener |timer_port: Port<TimerControlMsg>| {
            loop {
                match timer_port.recv() {
                  Close => break,
                  Fire(td) => {
                    content_chan.send(Timer(copy td));
                  }
                }
            }
        };
    }
    drop {
        self.timer_chan.send(Close);
    }
}

struct Document {
    let root: Node;
    let scope: NodeScope;
    let css_rules: ARC<Stylesheet>;

    new(root: Node, scope: NodeScope, -css_rules: Stylesheet) {
        self.root = root;
        self.scope = scope;
        self.css_rules = ARC(css_rules);
    }
}

enum NodeData = {
    tree: tree::Tree<Node>,
    kind: ~NodeKind,
};

enum NodeKind {
    Doctype(DoctypeData),
    Comment(~str),
    Element(ElementData),
    Text(~str)
}

struct DoctypeData {
    let name: ~str;
    let public_id: Option<~str>;
    let system_id: Option<~str>;
    let force_quirks: bool;

    new (name: ~str, public_id: Option<~str>,
         system_id: Option<~str>, force_quirks: bool) {
        self.name = name;
        self.public_id = public_id;
        self.system_id = system_id;
        self.force_quirks = force_quirks;
    }
}

struct ElementData {
    let tag_name: ~str;
    let kind: ~ElementKind;
    let attrs: DVec<~Attr>;

    new(-tag_name: ~str, -kind: ~ElementKind) {
        self.tag_name = tag_name;
        self.kind = kind;
        self.attrs = DVec();
    }

    fn get_attr(attr_name: ~str) -> Option<~str> {
        let mut i = 0u;
        while i < self.attrs.len() {
            if attr_name == self.attrs[i].name {
                return Some(copy self.attrs[i].value);
            }
            i += 1u;
        }

        None
    }
}

struct Attr {
    let name: ~str;
    let value: ~str;

    new(-name: ~str, -value: ~str) {
        self.name = name;
        self.value = value;
    }
}

fn define_bindings(compartment: bare_compartment, doc: @Document,
                   win: @Window) {
    bindings::window::init(compartment, win);
    bindings::document::init(compartment, doc);
    bindings::node::init(compartment);
    bindings::element::init(compartment);
}

enum ElementKind {
    UnknownElement,
    HTMLDivElement,
    HTMLHeadElement,
    HTMLImageElement({mut size: Size2D<au>}),
    HTMLScriptElement
}

#[doc="
    The rd_aux data is a (weak) pointer to the layout data, which contains the CSS info as well as
    the primary box.  Note that there may be multiple boxes per DOM node.
"]

type Node = rcu::Handle<NodeData, LayoutData>;

type NodeScope = rcu::Scope<NodeData, LayoutData>;

fn NodeScope() -> NodeScope {
    rcu::Scope()
}

trait NodeScopeExtensions {
    fn new_node(-k: NodeKind) -> Node;
}

#[allow(non_implicitly_copyable_typarams)]
impl NodeScope : NodeScopeExtensions {
    fn new_node(-k: NodeKind) -> Node {
        self.handle(NodeData({tree: tree::empty(), kind: ~k}))
    }
}

#[allow(non_implicitly_copyable_typarams)]
impl NodeScope : tree::ReadMethods<Node> {
    fn each_child(node: Node, f: fn(Node) -> bool) {
        tree::each_child(self, node, f)
    }

    fn get_parent(node: Node) -> Option<Node> {
        tree::get_parent(self, node)
    }

    fn with_tree_fields<R>(node: Node, f: fn(tree::Tree<Node>) -> R) -> R {
        self.read(node, |n| f(n.tree))
    }
}

#[allow(non_implicitly_copyable_typarams)]
impl NodeScope : tree::WriteMethods<Node> {
    fn add_child(node: Node, child: Node) {
        tree::add_child(self, node, child)
    }

    fn with_tree_fields<R>(node: Node, f: fn(tree::Tree<Node>) -> R) -> R {
        self.write(node, |n| f(n.tree))
    }
}


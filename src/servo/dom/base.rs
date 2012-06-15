import dom::rcu::{writer_methods};
import gfx::geometry::au;
import geom::size::Size2D;
import layout::base::layout_data;
import util::tree;
import dvec::{dvec, extensions};

enum node_data = {
    tree: tree::fields<Node>,
    kind: ~node_kind,
};

enum node_kind {
    Element(ElementData),
    Text(str)
}

class ElementData {
    let tag_name: str;
    let kind: ~ElementKind;
    let attrs: dvec<~attr>;

    new(-tag_name: str, -kind: ~ElementKind) {
        self.tag_name = tag_name;
        self.kind = kind;
        self.attrs = dvec();
    }

    fn get_attr(attr_name: str) -> option<str> {
        let mut i = 0u;
        while i < self.attrs.len() {
            if attr_name == self.attrs[i].name {
                ret some(copy self.attrs[i].value);
            }
            i += 1u;
        }

        none
    }
}

class attr {
    let name: str;
    let value: str;

    new(-name: str, -value: str) {
        self.name = name;
        self.value = value;
    }
}

enum ElementKind {
    UnknownElement,
    HTMLDivElement,
    HTMLHeadElement,
    HTMLImageElement({mut size: Size2D<au>})
}

#[doc="
    The rd_aux data is a (weak) pointer to the layout data, which contains the CSS info as well as
    the primary box.  Note that there may be multiple boxes per DOM node.
"]

type Node = rcu::handle<node_data, layout_data>;

type node_scope = rcu::scope<node_data, layout_data>;

fn node_scope() -> node_scope { rcu::scope() }

impl methods for node_scope {
    fn new_node(-k: node_kind) -> Node {
        self.handle(node_data({tree: tree::empty(),
                               kind: ~k}))
    }
}

impl of tree::rd_tree_ops<Node> for node_scope {
    fn each_child(node: Node, f: fn(Node) -> bool) {
        tree::each_child(self, node, f)
    }

    fn get_parent(node: Node) -> option<Node> {
        tree::get_parent(self, node)
    }

    fn with_tree_fields<R>(node: Node, f: fn(tree::fields<Node>) -> R) -> R {
        self.rd(node) { |n| f(n.tree) }
    }
}

impl of tree::wr_tree_ops<Node> for node_scope {
    fn add_child(node: Node, child: Node) {
        tree::add_child(self, node, child)
    }

    fn with_tree_fields<R>(node: Node, f: fn(tree::fields<Node>) -> R) -> R {
        self.wr(node) { |n| f(n.tree) }
    }
}


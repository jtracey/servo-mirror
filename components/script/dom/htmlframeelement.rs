/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::codegen::Bindings::HTMLFrameElementBinding;
use dom::bindings::codegen::InheritTypes::{ElementTypeId, EventTargetTypeId};
use dom::bindings::codegen::InheritTypes::{HTMLElementTypeId, HTMLFrameElementDerived, NodeTypeId};
use dom::bindings::js::Root;
use dom::document::Document;
use dom::eventtarget::EventTarget;
use dom::htmlelement::HTMLElement;
use dom::node::Node;
use util::str::DOMString;

#[dom_struct]
pub struct HTMLFrameElement {
    htmlelement: HTMLElement
}

impl HTMLFrameElementDerived for EventTarget {
    fn is_htmlframeelement(&self) -> bool {
        *self.type_id() ==
            EventTargetTypeId::Node(
                NodeTypeId::Element(ElementTypeId::HTMLElement(HTMLElementTypeId::HTMLFrameElement)))
    }
}

impl HTMLFrameElement {
    fn new_inherited(localName: DOMString, prefix: Option<DOMString>, document: &Document) -> HTMLFrameElement {
        HTMLFrameElement {
            htmlelement: HTMLElement::new_inherited(HTMLElementTypeId::HTMLFrameElement, localName, prefix, document)
        }
    }

    #[allow(unrooted_must_root)]
    pub fn new(localName: DOMString,
               prefix: Option<DOMString>,
               document: &Document) -> Root<HTMLFrameElement> {
        let element = HTMLFrameElement::new_inherited(localName, prefix, document);
        Node::reflect_node(box element, document, HTMLFrameElementBinding::Wrap)
    }
}

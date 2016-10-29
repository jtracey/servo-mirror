/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! A safe wrapper for DOM nodes that prevents layout from mutating the DOM, from letting DOM nodes
//! escape, and from generally doing anything that it isn't supposed to. This is accomplished via
//! a simple whitelist of allowed operations, along with some lifetime magic to prevent nodes from
//! escaping.
//!
//! As a security wrapper is only as good as its whitelist, be careful when adding operations to
//! this list. The cardinal rules are:
//!
//! 1. Layout is not allowed to mutate the DOM.
//!
//! 2. Layout is not allowed to see anything with `LayoutJS` in the name, because it could hang
//!    onto these objects and cause use-after-free.
//!
//! When implementing wrapper functions, be careful that you do not touch the borrow flags, or you
//! will race and cause spurious thread failure. (Note that I do not believe these races are
//! exploitable, but they'll result in brokenness nonetheless.)
//!
//! Rules of the road for this file:
//!
//! * Do not call any methods on DOM nodes without checking to see whether they use borrow flags.
//!
//!   o Instead of `get_attr()`, use `.get_attr_val_for_layout()`.
//!
//!   o Instead of `html_element_in_html_document()`, use
//!     `html_element_in_html_document_for_layout()`.

#![allow(unsafe_code)]

use core::nonzero::NonZero;
use data::{LayoutDataFlags, PersistentLayoutData};
use script_layout_interface::{OpaqueStyleAndLayoutData, PartialPersistentLayoutData};
use script_layout_interface::wrapper_traits::{ThreadSafeLayoutElement, ThreadSafeLayoutNode};
use script_layout_interface::wrapper_traits::GetLayoutData;
use style::atomic_refcell::{AtomicRef, AtomicRefCell, AtomicRefMut};
use style::computed_values::content::{self, ContentItem};

pub type NonOpaqueStyleAndLayoutData = AtomicRefCell<PersistentLayoutData>;

pub trait LayoutNodeLayoutData {
    /// Similar to borrow_data*, but returns the full PersistentLayoutData rather
    /// than only the style::data::ElementData.
    fn borrow_layout_data(&self) -> Option<AtomicRef<PersistentLayoutData>>;
    fn mutate_layout_data(&self) -> Option<AtomicRefMut<PersistentLayoutData>>;
    fn flow_debug_id(self) -> usize;
}

impl<T: GetLayoutData> LayoutNodeLayoutData for T {
    fn borrow_layout_data(&self) -> Option<AtomicRef<PersistentLayoutData>> {
        self.get_raw_data().map(|d| d.borrow())
    }

    fn mutate_layout_data(&self) -> Option<AtomicRefMut<PersistentLayoutData>> {
        self.get_raw_data().map(|d| d.borrow_mut())
    }

    fn flow_debug_id(self) -> usize {
        self.borrow_layout_data().map_or(0, |d| d.flow_construction_result.debug_id())
    }
}

pub trait LayoutNodeHelpers {
    fn initialize_data(&self);
    fn get_raw_data(&self) -> Option<&NonOpaqueStyleAndLayoutData>;
}

impl<T: GetLayoutData> LayoutNodeHelpers for T {
    fn initialize_data(&self) {
        if self.get_raw_data().is_none() {
            let ptr: *mut NonOpaqueStyleAndLayoutData =
                Box::into_raw(box AtomicRefCell::new(PersistentLayoutData::new()));
            let opaque = OpaqueStyleAndLayoutData {
                ptr: unsafe { NonZero::new(ptr as *mut AtomicRefCell<PartialPersistentLayoutData>) }
            };
            self.init_style_and_layout_data(opaque);
        };
    }

    fn get_raw_data(&self) -> Option<&NonOpaqueStyleAndLayoutData> {
        self.get_style_and_layout_data().map(|opaque| {
            let container = *opaque.ptr as *mut NonOpaqueStyleAndLayoutData;
            unsafe { &*container }
        })
    }
}

pub trait ThreadSafeLayoutNodeHelpers {
    /// Returns the layout data flags for this node.
    fn flags(self) -> LayoutDataFlags;

    /// Adds the given flags to this node.
    fn insert_flags(self, new_flags: LayoutDataFlags);

    /// Removes the given flags from this node.
    fn remove_flags(self, flags: LayoutDataFlags);

    /// If this is a text node, generated content, or a form element, copies out
    /// its content. Otherwise, panics.
    ///
    /// FIXME(pcwalton): This might have too much copying and/or allocation. Profile this.
    fn text_content(&self) -> TextContent;
}

impl<T: ThreadSafeLayoutNode> ThreadSafeLayoutNodeHelpers for T {
    fn flags(self) -> LayoutDataFlags {
            self.borrow_layout_data().as_ref().unwrap().flags
    }

    fn insert_flags(self, new_flags: LayoutDataFlags) {
        self.mutate_layout_data().unwrap().flags.insert(new_flags);
    }

    fn remove_flags(self, flags: LayoutDataFlags) {
        self.mutate_layout_data().unwrap().flags.remove(flags);
    }

    fn text_content(&self) -> TextContent {
        if self.get_pseudo_element_type().is_replaced_content() {
            let style = self.as_element().unwrap().resolved_style();

            return match style.as_ref().get_counters().content {
                content::T::Content(ref value) if !value.is_empty() => {
                    TextContent::GeneratedContent((*value).clone())
                }
                _ => TextContent::GeneratedContent(vec![]),
            };
        }

        return TextContent::Text(self.node_text_content());
    }
}

pub enum TextContent {
    Text(String),
    GeneratedContent(Vec<ContentItem>),
}

impl TextContent {
    pub fn is_empty(&self) -> bool {
        match *self {
            TextContent::Text(_) => false,
            TextContent::GeneratedContent(ref content) => content.is_empty(),
        }
    }
}

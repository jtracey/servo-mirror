// Copyright (c) 2015 Marshall A. Greenblatt. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the name Chromium Embedded
// Framework nor the names of its contributors may be used to endorse
// or promote products derived from this software without specific prior
// written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// ---------------------------------------------------------------------------
//
// This file was generated by the CEF translator tool and should not be edited
// by hand. See the translator.README.txt file in the tools directory for
// more information.
//

#![allow(non_snake_case, unused_imports)]

use eutil;
use interfaces;
use types;
use wrappers::CefWrap;

use libc;
use std::collections::HashMap;
use std::mem;
use std::ptr;

//
// Implement this structure to handle events related to dragging. The functions
// of this structure will be called on the UI thread.
//
#[repr(C)]
pub struct _cef_drag_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Called when an external drag event enters the browser window. |dragData|
  // contains the drag event data and |mask| represents the type of drag
  // operation. Return false (0) for default drag handling behavior or true (1)
  // to cancel the drag event.
  //
  pub on_drag_enter: Option<extern "C" fn(this: *mut cef_drag_handler_t,
      browser: *mut interfaces::cef_browser_t,
      dragData: *mut interfaces::cef_drag_data_t,
      mask: types::cef_drag_operations_mask_t) -> libc::c_int>,

  //
  // Called whenever draggable regions for the browser window change. These can
  // be specified using the '-webkit-app-region: drag/no-drag' CSS-property. If
  // draggable regions are never defined in a document this function will also
  // never be called. If the last draggable region is removed from a document
  // this function will be called with an NULL vector.
  //
  pub on_draggable_regions_changed: Option<extern "C" fn(
      this: *mut cef_drag_handler_t, browser: *mut interfaces::cef_browser_t,
      regions_count: libc::size_t,
      regions: *const types::cef_draggable_region_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: u32,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_drag_handler_t = _cef_drag_handler_t;


//
// Implement this structure to handle events related to dragging. The functions
// of this structure will be called on the UI thread.
//
pub struct CefDragHandler {
  c_object: *mut cef_drag_handler_t,
}

impl Clone for CefDragHandler {
  fn clone(&self) -> CefDragHandler{
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefDragHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefDragHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefDragHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_drag_handler_t) -> CefDragHandler {
    CefDragHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_drag_handler_t) -> CefDragHandler {
    if !c_object.is_null() &&
        c_object as usize != mem::POST_DROP_USIZE {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefDragHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_drag_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_drag_handler_t {
    unsafe {
      if !self.c_object.is_null() &&
          self.c_object as usize != mem::POST_DROP_USIZE {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null() || self.c_object as usize == mem::POST_DROP_USIZE
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null() && self.c_object as usize != mem::POST_DROP_USIZE
  }

  //
  // Called when an external drag event enters the browser window. |dragData|
  // contains the drag event data and |mask| represents the type of drag
  // operation. Return false (0) for default drag handling behavior or true (1)
  // to cancel the drag event.
  //
  pub fn on_drag_enter(&self, browser: interfaces::CefBrowser,
      dragData: interfaces::CefDragData,
      mask: types::cef_drag_operations_mask_t) -> libc::c_int {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_drag_enter.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(dragData),
          CefWrap::to_c(mask)))
    }
  }

  //
  // Called whenever draggable regions for the browser window change. These can
  // be specified using the '-webkit-app-region: drag/no-drag' CSS-property. If
  // draggable regions are never defined in a document this function will also
  // never be called. If the last draggable region is removed from a document
  // this function will be called with an NULL vector.
  //
  pub fn on_draggable_regions_changed(&self, browser: interfaces::CefBrowser,
      regions_count: libc::size_t,
      regions: *const types::cef_draggable_region_t) -> () {
    if self.c_object.is_null() ||
       self.c_object as usize == mem::POST_DROP_USIZE {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_draggable_regions_changed.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(regions_count),
          CefWrap::to_c(regions)))
    }
  }
} 

impl CefWrap<*mut cef_drag_handler_t> for CefDragHandler {
  fn to_c(rust_object: CefDragHandler) -> *mut cef_drag_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_drag_handler_t) -> CefDragHandler {
    CefDragHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_drag_handler_t> for Option<CefDragHandler> {
  fn to_c(rust_object: Option<CefDragHandler>) -> *mut cef_drag_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_drag_handler_t) -> Option<CefDragHandler> {
    if c_object.is_null() &&
       c_object as usize != mem::POST_DROP_USIZE {
      None
    } else {
      Some(CefDragHandler::from_c_object_addref(c_object))
    }
  }
}


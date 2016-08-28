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
// Implement this structure to handle events related to browser display state.
// The functions of this structure will be called on the UI thread.
//
#[repr(C)]
pub struct _cef_display_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Called when a frame's address has changed.
  //
  pub on_address_change: Option<extern "C" fn(this: *mut cef_display_handler_t,
      browser: *mut interfaces::cef_browser_t,
      frame: *mut interfaces::cef_frame_t,
      url: *const types::cef_string_t) -> ()>,

  //
  // Called when the page title changes.
  //
  pub on_title_change: Option<extern "C" fn(this: *mut cef_display_handler_t,
      browser: *mut interfaces::cef_browser_t,
      title: *const types::cef_string_t) -> ()>,

  //
  // Called when the page icon changes.
  //
  pub on_favicon_urlchange: Option<extern "C" fn(
      this: *mut cef_display_handler_t, browser: *mut interfaces::cef_browser_t,
      icon_urls: &types::cef_string_list_t) -> ()>,

  //
  // Called when web content in the page has toggled fullscreen mode. If
  // |fullscreen| is true (1) the content will automatically be sized to fill
  // the browser content area. If |fullscreen| is false (0) the content will
  // automatically return to its original size and position. The client is
  // responsible for resizing the browser if desired.
  //
  pub on_fullscreen_mode_change: Option<extern "C" fn(
      this: *mut cef_display_handler_t, browser: *mut interfaces::cef_browser_t,
      fullscreen: libc::c_int) -> ()>,

  //
  // Called when the browser is about to display a tooltip. |text| contains the
  // text that will be displayed in the tooltip. To handle the display of the
  // tooltip yourself return true (1). Otherwise, you can optionally modify
  // |text| and then return false (0) to allow the browser to display the
  // tooltip. When window rendering is disabled the application is responsible
  // for drawing tooltips and the return value is ignored.
  //
  pub on_tooltip: Option<extern "C" fn(this: *mut cef_display_handler_t,
      browser: *mut interfaces::cef_browser_t,
      text: *mut types::cef_string_t) -> libc::c_int>,

  //
  // Called when the browser receives a status message. |value| contains the
  // text that will be displayed in the status message.
  //
  pub on_status_message: Option<extern "C" fn(this: *mut cef_display_handler_t,
      browser: *mut interfaces::cef_browser_t,
      value: *const types::cef_string_t) -> ()>,

  //
  // Called to display a console message. Return true (1) to stop the message
  // from being output to the console.
  //
  pub on_console_message: Option<extern "C" fn(this: *mut cef_display_handler_t,
      browser: *mut interfaces::cef_browser_t,
      message: *const types::cef_string_t, source: *const types::cef_string_t,
      line: libc::c_int) -> libc::c_int>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: u32,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
}

pub type cef_display_handler_t = _cef_display_handler_t;


//
// Implement this structure to handle events related to browser display state.
// The functions of this structure will be called on the UI thread.
//
pub struct CefDisplayHandler {
  c_object: *mut cef_display_handler_t,
}

impl Clone for CefDisplayHandler {
  fn clone(&self) -> CefDisplayHandler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefDisplayHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefDisplayHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefDisplayHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_display_handler_t) -> CefDisplayHandler {
    CefDisplayHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_display_handler_t) -> CefDisplayHandler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefDisplayHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_display_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_display_handler_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Called when a frame's address has changed.
  //
  pub fn on_address_change(&self, browser: interfaces::CefBrowser,
      frame: interfaces::CefFrame, url: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_address_change.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(frame),
          CefWrap::to_c(url)))
    }
  }

  //
  // Called when the page title changes.
  //
  pub fn on_title_change(&self, browser: interfaces::CefBrowser,
      title: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_title_change.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(title)))
    }
  }

  //
  // Called when the page icon changes.
  //
  pub fn on_favicon_urlchange(&self, browser: interfaces::CefBrowser,
      icon_urls: &Vec<String>) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_favicon_urlchange.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(icon_urls)))
    }
  }

  //
  // Called when web content in the page has toggled fullscreen mode. If
  // |fullscreen| is true (1) the content will automatically be sized to fill
  // the browser content area. If |fullscreen| is false (0) the content will
  // automatically return to its original size and position. The client is
  // responsible for resizing the browser if desired.
  //
  pub fn on_fullscreen_mode_change(&self, browser: interfaces::CefBrowser,
      fullscreen: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_fullscreen_mode_change.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(fullscreen)))
    }
  }

  //
  // Called when the browser is about to display a tooltip. |text| contains the
  // text that will be displayed in the tooltip. To handle the display of the
  // tooltip yourself return true (1). Otherwise, you can optionally modify
  // |text| and then return false (0) to allow the browser to display the
  // tooltip. When window rendering is disabled the application is responsible
  // for drawing tooltips and the return value is ignored.
  //
  pub fn on_tooltip(&self, browser: interfaces::CefBrowser,
      text: *mut types::cef_string_t) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_tooltip.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(text)))
    }
  }

  //
  // Called when the browser receives a status message. |value| contains the
  // text that will be displayed in the status message.
  //
  pub fn on_status_message(&self, browser: interfaces::CefBrowser,
      value: &[u16]) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_status_message.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(value)))
    }
  }

  //
  // Called to display a console message. Return true (1) to stop the message
  // from being output to the console.
  //
  pub fn on_console_message(&self, browser: interfaces::CefBrowser,
      message: &[u16], source: &[u16], line: libc::c_int) -> libc::c_int {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_console_message.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(message),
          CefWrap::to_c(source),
          CefWrap::to_c(line)))
    }
  }
} 

impl CefWrap<*mut cef_display_handler_t> for CefDisplayHandler {
  fn to_c(rust_object: CefDisplayHandler) -> *mut cef_display_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_display_handler_t) -> CefDisplayHandler {
    CefDisplayHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_display_handler_t> for Option<CefDisplayHandler> {
  fn to_c(rust_object: Option<CefDisplayHandler>) -> *mut cef_display_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_display_handler_t) -> Option<CefDisplayHandler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefDisplayHandler::from_c_object_addref(c_object))
    }
  }
}


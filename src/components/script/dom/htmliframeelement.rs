/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use dom::bindings::utils::{DOMString, null_string, ErrorResult, str};
use dom::document::AbstractDocument;
use dom::htmlelement::HTMLElement;
use dom::windowproxy::WindowProxy;
use geom::size::Size2D;
use geom::rect::Rect;

use servo_msg::constellation_msg::{ConstellationChan, FrameRectMsg, PipelineId, SubpageId};

use std::ascii::StrAsciiExt;
use std::comm::ChanOne;
use extra::url::Url;
use std::util::replace;

enum SandboxAllowance {
    AllowNothing = 0x00,
    AllowSameOrigin = 0x01,
    AllowTopNavigation = 0x02,
    AllowForms = 0x04,
    AllowScripts = 0x08,
    AllowPointerLock = 0x10,
    AllowPopups = 0x20
}

pub struct HTMLIFrameElement {
    parent: HTMLElement,
    frame: Option<Url>,
    size: Option<IFrameSize>,
    sandbox: Option<u8>
}

struct IFrameSize {
    pipeline_id: PipelineId,
    subpage_id: SubpageId,
    future_chan: Option<ChanOne<Size2D<uint>>>,
    constellation_chan: ConstellationChan,
}

impl IFrameSize {
    pub fn set_rect(&mut self, rect: Rect<f32>) {
        let future_chan = replace(&mut self.future_chan, None);
        do future_chan.map_move |future_chan| {
            let Size2D { width, height } = rect.size;
            future_chan.send(Size2D(width as uint, height as uint));
        };
        self.constellation_chan.send(FrameRectMsg(self.pipeline_id, self.subpage_id, rect));
    }
}

impl HTMLIFrameElement {
    pub fn is_sandboxed(&self) -> bool {
        self.sandbox.is_some()
    }
}

impl HTMLIFrameElement {
    pub fn Src(&self) -> DOMString {
        null_string
    }

    pub fn SetSrc(&mut self, _src: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn Srcdoc(&self) -> DOMString {
        null_string
    }

    pub fn SetSrcdoc(&mut self, _srcdoc: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn Name(&self) -> DOMString {
        null_string
    }

    pub fn SetName(&mut self, _name: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn Sandbox(&self) -> DOMString {
        self.parent.parent.GetAttribute(&str(~"sandbox"))
    }

    pub fn SetSandbox(&mut self, sandbox: &DOMString) {
        let mut rv = Ok(());
        self.parent.parent.SetAttribute(&str(~"sandbox"), sandbox, &mut rv);
    }

    pub fn AfterSetAttr(&mut self, name: &DOMString, value: &DOMString) {
        let name = name.to_str();
        if "sandbox" == name {
            let mut modes = AllowNothing as u8;
            let words = value.to_str();
            for word in words.split_iter(' ') {
                modes |= match word.to_ascii_lower().as_slice() {
                    "allow-same-origin" => AllowSameOrigin,
                    "allow-forms" => AllowForms,
                    "allow-pointer-lock" => AllowPointerLock,
                    "allow-popups" => AllowPopups,
                    "allow-scripts" => AllowScripts,
                    "allow-top-navigation" => AllowTopNavigation,
                    _ => AllowNothing
                } as u8;
            }
            self.sandbox = Some(modes);
        }
    }

    pub fn AllowFullscreen(&self) -> bool {
        false
    }

    pub fn SetAllowFullscreen(&mut self, _allow: bool, _rv: &mut ErrorResult) {
    }

    pub fn Width(&self) -> DOMString {
        null_string
    }

    pub fn SetWidth(&mut self, _width: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn Height(&self) -> DOMString {
        null_string
    }

    pub fn SetHeight(&mut self, _height: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn GetContentDocument(&self) -> Option<AbstractDocument> {
        None
    }

    pub fn GetContentWindow(&self) -> Option<@mut WindowProxy> {
        None
    }

    pub fn Align(&self) -> DOMString {
        null_string
    }

    pub fn SetAlign(&mut self, _align: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn Scrolling(&self) -> DOMString {
        null_string
    }

    pub fn SetScrolling(&mut self, _scrolling: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn FrameBorder(&self) -> DOMString {
        null_string
    }

    pub fn SetFrameBorder(&mut self, _frameborder: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn LongDesc(&self) -> DOMString {
        null_string
    }

    pub fn SetLongDesc(&mut self, _longdesc: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn MarginHeight(&self) -> DOMString {
        null_string
    }

    pub fn SetMarginHeight(&mut self, _marginheight: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn MarginWidth(&self) -> DOMString {
        null_string
    }

    pub fn SetMarginWidth(&mut self, _marginwidth: &DOMString, _rv: &mut ErrorResult) {
    }

    pub fn GetSVGDocument(&self) -> Option<AbstractDocument> {
        None
    }
}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use gfx::font_cache_task::FontCacheTask;
use ipc_channel::ipc;
use style::computed_values::font_family::FontFamily;
use style::font_face::Source;

#[test]
fn test_local_web_font() {
  let (inp_chan, _) = ipc::channel().unwrap();
  let (out_chan, out_receiver) = ipc::channel().unwrap();
  let font_cache_task = FontCacheTask::new(inp_chan);
  let family_name = FontFamily::FamilyName(From::from("test family"));
  let variant_name = FontFamily::FamilyName(From::from("test font face"));

  font_cache_task.add_web_font(family_name, Source::Local(variant_name), out_chan);

  assert_eq!(out_receiver.recv().unwrap(), ());
}

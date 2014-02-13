/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::task;
use std::comm::SharedChan;
use std::task::TaskBuilder;

pub fn spawn_named<S: IntoSendStr>(name: S, f: proc()) {
    let mut builder = task::task();
    builder.name(name);
    builder.spawn(f);
}

/// Arrange to send a particular message to a channel if the task built by
/// this `TaskBuilder` fails.
pub fn send_on_failure<T: Send>(builder: &mut TaskBuilder, msg: T, dest: SharedChan<T>) {
    let port = builder.future_result();
    do spawn {
        match port.recv() {
            Ok(()) => (),
            Err(..) => dest.send(msg),
        }
    }
}

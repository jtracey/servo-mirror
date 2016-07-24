/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use app_units::Au;
use euclid::point::Point2D;
use euclid::rect::Rect;
use gfx_traits::{Epoch, LayerId};
use ipc_channel::ipc::{IpcReceiver, IpcSender};
use msg::constellation_msg::PipelineId;
use net_traits::image_cache_thread::ImageCacheThread;
use profile_traits::mem::ReportsChan;
use rpc::LayoutRPC;
use script_traits::{ConstellationControlMsg, LayoutControlMsg};
use script_traits::{LayoutMsg as ConstellationMsg, StackingContextScrollState, WindowSizeData};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use string_cache::Atom;
use style::context::ReflowGoal;
use style::selector_impl::PseudoElement;
use style::stylesheets::Stylesheet;
use url::Url;
use util::ipc::OptionalOpaqueIpcSender;
use {OpaqueStyleAndLayoutData, TrustedNodeAddress};

/// Asynchronous messages that script can send to layout.
pub enum Msg {
    /// Adds the given stylesheet to the document.
    AddStylesheet(Arc<Stylesheet>),

    /// Puts a document into quirks mode, causing the quirks mode stylesheet to be loaded.
    SetQuirksMode,

    /// Requests a reflow.
    Reflow(ScriptReflow),

    /// Get an RPC interface.
    GetRPC(Sender<Box<LayoutRPC + Send>>),

    /// Requests that the layout thread render the next frame of all animations.
    TickAnimations,

    /// Updates layout's timer for animation testing from script.
    ///
    /// The inner field is the number of *milliseconds* to advance.
    AdvanceClockMs(i32),

    /// Requests that the layout thread reflow with a newly-loaded Web font.
    ReflowWithNewlyLoadedWebFont,

    /// Updates the layout visible rects, affecting the area that display lists will be constructed
    /// for.
    SetVisibleRects(Vec<(LayerId, Rect<Au>)>),

    /// Destroys layout data associated with a DOM node.
    ///
    /// TODO(pcwalton): Maybe think about batching to avoid message traffic.
    ReapStyleAndLayoutData(OpaqueStyleAndLayoutData),

    /// Requests that the layout thread measure its memory usage. The resulting reports are sent back
    /// via the supplied channel.
    CollectReports(ReportsChan),

    /// Requests that the layout thread enter a quiescent state in which no more messages are
    /// accepted except `ExitMsg`. A response message will be sent on the supplied channel when
    /// this happens.
    PrepareToExit(Sender<()>),

    /// Requests that the layout thread immediately shut down. There must be no more nodes left after
    /// this, or layout will crash.
    ExitNow,

    /// Get the last epoch counter for this layout thread.
    GetCurrentEpoch(IpcSender<Epoch>),

    /// Asks the layout thread whether any Web fonts have yet to load (if true, loads are pending;
    /// false otherwise).
    GetWebFontLoadState(IpcSender<bool>),

    /// Creates a new layout thread.
    ///
    /// This basically exists to keep the script-layout dependency one-way.
    CreateLayoutThread(NewLayoutThreadInfo),

    /// Set the final Url.
    SetFinalUrl(Url),

    /// Tells layout about the new scrolling offsets of each scrollable stacking context.
    SetStackingContextScrollStates(Vec<StackingContextScrollState>),
}


/// Any query to perform with this reflow.
#[derive(PartialEq)]
pub enum ReflowQueryType {
    NoQuery,
    ContentBoxQuery(TrustedNodeAddress),
    ContentBoxesQuery(TrustedNodeAddress),
    NodeOverflowQuery(TrustedNodeAddress),
    HitTestQuery(Point2D<f32>, bool),
    NodeGeometryQuery(TrustedNodeAddress),
    NodeLayerIdQuery(TrustedNodeAddress),
    NodeScrollGeometryQuery(TrustedNodeAddress),
    ResolvedStyleQuery(TrustedNodeAddress, Option<PseudoElement>, Atom),
    OffsetParentQuery(TrustedNodeAddress),
    MarginStyleQuery(TrustedNodeAddress),
}

/// Information needed for a reflow.
pub struct Reflow {
    /// The goal of reflow: either to render to the screen or to flush layout info for script.
    pub goal: ReflowGoal,
    ///  A clipping rectangle for the page, an enlarged rectangle containing the viewport.
    pub page_clip_rect: Rect<Au>,
}

/// Information needed for a script-initiated reflow.
pub struct ScriptReflow {
    /// General reflow data.
    pub reflow_info: Reflow,
    /// The document node.
    pub document: TrustedNodeAddress,
    /// The document's list of stylesheets.
    pub document_stylesheets: Vec<Arc<Stylesheet>>,
    /// Whether the document's stylesheets have changed since the last script reflow.
    pub stylesheets_changed: bool,
    /// The current window size.
    pub window_size: WindowSizeData,
    /// The channel that we send a notification to.
    pub script_join_chan: Sender<()>,
    /// The type of query if any to perform during this reflow.
    pub query_type: ReflowQueryType,
}

impl Drop for ScriptReflow {
    fn drop(&mut self) {
        self.script_join_chan.send(()).unwrap();
    }
}

pub struct NewLayoutThreadInfo {
    pub id: PipelineId,
    pub url: Url,
    pub is_parent: bool,
    pub layout_pair: (Sender<Msg>, Receiver<Msg>),
    pub pipeline_port: IpcReceiver<LayoutControlMsg>,
    pub constellation_chan: IpcSender<ConstellationMsg>,
    pub script_chan: IpcSender<ConstellationControlMsg>,
    pub image_cache_thread: ImageCacheThread,
    pub paint_chan: OptionalOpaqueIpcSender,
    pub content_process_shutdown_chan: IpcSender<()>,
    pub layout_threads: usize,
}

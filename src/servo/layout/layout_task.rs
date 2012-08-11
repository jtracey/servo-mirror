#[doc = "
    The layout task. Performs layout on the DOM, builds display lists and sends them to be
    rendered.
"];

import std::arc::arc;
import display_list_builder::build_display_list;
import dom::base::Node;
import dom::style::Stylesheet;
import gfx::geometry::px_to_au;
import gfx::renderer::Renderer;
import resource::image_cache_task::ImageCacheTask;
import std::net::url::url;
import style::apply::apply_style;

import task::*;
import comm::*;

type Layout = chan<Msg>;

enum Msg {
    BuildMsg(Node, arc<Stylesheet>, url),
    PingMsg(chan<content::PingMsg>),
    ExitMsg
}

fn Layout(renderer: Renderer, image_cache_task: ImageCacheTask) -> Layout {
    do spawn_listener::<Msg>|request| {
        loop {
            match request.recv() {
              PingMsg(ping_channel) => ping_channel.send(content::PongMsg),
              ExitMsg => {
                #debug("layout: ExitMsg received");
                break;
              }
              BuildMsg(node, styles, doc_url) => {
                #debug("layout: received layout request for:");
                node.dump();

                do util::time::time(~"layout") {
                    node.initialize_style_for_subtree();
                    node.recompute_style_for_subtree(styles);

                    let this_box = node.construct_boxes();
                    this_box.dump();

                    apply_style(this_box, &doc_url, image_cache_task);

                    this_box.reflow_subtree(px_to_au(800));

                    let dlist = build_display_list(this_box);
                    renderer.send(renderer::RenderMsg(dlist));
                }
              }
            }
        }
    }
}

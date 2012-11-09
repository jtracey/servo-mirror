//! Configuration options for a single run of the servo application. Created
//! from command line arguments.

use azure::azure_hl::{BackendType, CairoBackend, CoreGraphicsBackend};
use azure::azure_hl::{CoreGraphicsAcceleratedBackend, Direct2DBackend, SkiaBackend};

pub struct Opts {
    urls: ~[~str],
    render_mode: RenderMode,
    render_backend: BackendType
}

pub enum RenderMode {
    Screen,
    Png(~str)
}

#[allow(non_implicitly_copyable_typarams)]
pub fn from_cmdline_args(args: &[~str]) -> Opts {
    use std::getopts;

    let args = args.tail();

    let opts = ~[
        getopts::optopt(~"o"),
        getopts::optopt(~"r")
    ];

    let opt_match = match getopts::getopts(args, opts) {
      result::Ok(m) => { copy m }
      result::Err(f) => { fail getopts::fail_str(copy f) }
    };

    let urls = if opt_match.free.is_empty() {
        fail ~"servo asks that you provide 1 or more URLs"
    } else {
        copy opt_match.free
    };

    let render_mode = match getopts::opt_maybe_str(copy opt_match, ~"o") {
      Some(move output_file) => { Png(move output_file) }
      None => { Screen }
    };

    let render_backend = match getopts::opt_maybe_str(move opt_match, ~"r") {
        Some(move backend_str) => {
            if backend_str == ~"direct2d" {
                Direct2DBackend
            } else if backend_str == ~"core-graphics" {
                CoreGraphicsBackend
            } else if backend_str == ~"core-graphics-accelerated" {
                CoreGraphicsAcceleratedBackend
            } else if backend_str == ~"cairo" {
                CairoBackend
            } else if backend_str == ~"skia" {
                SkiaBackend
            } else {
                fail ~"unknown backend type"
            }
        }
        None => CairoBackend
    };

    Opts {
        urls: move urls,
        render_mode: move render_mode,
        render_backend: move render_backend,
    }
}

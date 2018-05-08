#[macro_use] extern crate objc;
extern crate cocoa;

mod platform;
// pub use platform::*;

use std::os::raw::c_void;

pub fn run<
    ICB: FnOnce(),
    CB: FnOnce(),
    > (handle: *mut c_void, content: &str, init_callback: ICB, event_callback: CB) {

    let mut webview = platform::WebView::new(handle).unwrap();
    webview.load_html_string(content);

    init_callback();
    event_callback();
}

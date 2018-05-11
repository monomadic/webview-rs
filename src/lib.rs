#[macro_use] extern crate objc;
extern crate cocoa;

mod platform;
pub use platform::*;

use std::os::raw::c_void;

pub fn run<
    ICB: FnOnce(WebView),
    CB: 'static + FnMut(WebView, String, String),
    > (handle: *mut c_void, content: &str, init_callback: ICB, event_callback: CB) -> Result<(), String> {

    platform::WebView::new(
        handle, content, init_callback, event_callback).unwrap();

    Ok(())
}

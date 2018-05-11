#[macro_use] extern crate objc;
extern crate cocoa;

mod platform;
pub use platform::*;

use std::os::raw::c_void;

pub fn run<
    CB: 'static + FnMut(WebView, String, String),
    > (handle: *mut c_void, content: &str, event_callback: CB) -> Result<(), String> {

    platform::WebView::new(
        handle, content, event_callback).unwrap();

    Ok(())
}

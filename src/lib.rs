#[macro_use] extern crate objc;
extern crate cocoa;

mod platform;
pub use platform::*;

use std::os::raw::c_void;

pub fn run(
        handle: *mut c_void,
        content: &str,
        event_callback: impl FnMut(WebView, String, String) + 'static) -> Result<(), String> {

    platform::WebView::new(
        handle, content, event_callback).unwrap();

    Ok(())
}

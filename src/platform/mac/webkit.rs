#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use cocoa;
use cocoa::base::{ id, nil, NO, YES, class };
use cocoa::foundation::{ NSString };
use cocoa::foundation::{ NSRect, NSPoint, NSSize };
use cocoa::appkit::{ NSView, NSWindow };

use objc::runtime::{ Class, Object, Protocol, Sel };
use objc::declare::{ ClassDecl };
use objc;

#[link(name = "WebKit", kind = "framework")]
extern {
    pub static WKScriptMessageHandler: id;
}

#[derive(Copy, Clone)]
pub struct WebView {
    id: id,
}

fn nsstring_to_str(string: id) -> String {
    let bytes = unsafe {
        let bytes: *const ::std::os::raw::c_char = msg_send![string, UTF8String];
        bytes as *const u8
    };
    let len = unsafe { string.len() };
    unsafe {
        let bytes = ::std::slice::from_raw_parts(bytes, len);
        String::from_utf8(bytes.to_vec()).unwrap()
    }
}

impl WebView {
    pub fn new(window: *mut ::std::os::raw::c_void) -> Result<Self, String> {
        unsafe {

            // WKWebViewConfiguration
            let cls = Class::get("WKWebViewConfiguration").expect("WKWebViewConfiguration to exist");
            let configuration = {
                let obj: *mut Object = msg_send![cls, alloc];
                let obj: *mut Object = msg_send![obj, init];
                obj
            };

            let view = NSWindow::contentView(window as id);
            let window_frame = NSView::frame(view as id);

            // WKWebView
            let cls = Class::get("WKWebView").expect("WKWebView to exist");
            let webview = {
                let obj: *mut Object = msg_send![cls, alloc];
                let obj: *mut Object = msg_send![obj,
                    initWithFrame: window_frame
                    configuration: configuration ];
                obj
            };

            NSView::addSubview_(view, webview);
            // NSWindow::addView_(window as id, webview);

            Ok(WebView {
                id: webview
            })
        }
    }

    pub fn load_html_string(&mut self, html: &str) {
        unsafe {
            let cls = Class::get("NSURL").unwrap();
            let nsurl = {
                let obj: *mut Object = msg_send![cls, fileURLWithPath:NSString::alloc(nil).init_str("")];
                obj
            };

            msg_send![self.id,
                loadHTMLString:NSString::alloc(nil).init_str(html)
                baseURL:nsurl];
                
            msg_send![self.id, setOpaque:NO];
            // msg_send![self.id, setBackgroundColor:Color::clear().nscolor()];
        }
    }
}
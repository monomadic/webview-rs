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

use std::os::raw::c_void;

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

pub fn wk_script_message_handler_class() -> &'static Class {
    use std::sync::{Once, ONCE_INIT};

    static REGISTER_CUSTOM_SUBCLASS: Once = ONCE_INIT;
    REGISTER_CUSTOM_SUBCLASS.call_once(|| {
        let superclass = Class::get("WKUserContentController").unwrap();
        let mut decl = ClassDecl::new("NotificationScriptMessageHandler", superclass).unwrap();

        extern fn userContentController(this: &mut Object, _cmd: Sel, didReceive: bool, message: id) {
            let name = nsstring_to_str(unsafe { msg_send![message, name] });
            let body = nsstring_to_str(unsafe { msg_send![message, body] });

            let webview = unsafe { msg_send![message, webView] };
            ::send_event(webview, format!("name: {}, body: {}", name, body));
        }

        unsafe {
            decl.add_method(sel!(userContentController:didReceiveScriptMessage:),
                userContentController as extern fn(&mut Object, Sel, bool, id));
        }

        decl.register();
    });

    Class::get("NotificationScriptMessageHandler").expect("NotificationScriptMessageHandler to be valid.")
}

pub fn navigation_delegate_class() -> &'static Class {
    use std::sync::{Once, ONCE_INIT};

    static REGISTER_CUSTOM_SUBCLASS: Once = ONCE_INIT;
    REGISTER_CUSTOM_SUBCLASS.call_once(|| {
        let superclass = Class::get("WKWebView").expect("WKWebView to be available");
        let mut decl = ClassDecl::new("NavigationDelegate", superclass).expect("WKWebView to be subclassable");

        decl.add_protocol(Protocol::get("WKNavigationDelegate").expect("WKNavigationDelegate protocol to exist"));

        extern fn didCommitNavigation(this: &Object, _cmd: Sel, webview: id, navigation: id) {
            ::send_event(webview as *mut c_void, "commit nav".to_string());
        }
        extern fn didFinishNavigation(this: &Object, _cmd: Sel, webview: id, navigation: id) {
            ::send_event(webview as *mut c_void, "finished loading".to_string());
        }

        unsafe {
            decl.add_method(sel!(webView:didCommitNavigation:),
                didCommitNavigation as extern fn(&Object, Sel, id, id));
            decl.add_method(sel!(webView:didFinishNavigation:),
                didFinishNavigation as extern fn(&Object, Sel, id, id));
        }

        decl.register();
    });

    Class::get("NavigationDelegate").expect("NavigationDelegate to be valid.")
}

impl WebView {
    pub fn new(window: *mut ::std::os::raw::c_void) -> Result<Self, String> {
        unsafe {

            // WKUserContentController
            let cls = wk_script_message_handler_class();
            let scripthandler = {
                let obj: *mut Object = msg_send![cls, alloc];
                let obj: *mut Object = msg_send![obj, init];
                obj
            };

            msg_send![scripthandler, addScriptMessageHandler:scripthandler name:NSString::alloc(nil).init_str("notification")];

            // WKWebViewConfiguration;
            let cls = Class::get("WKWebViewConfiguration").ok_or("WKWebViewConfiguration does not exist")?;
            let configuration = {
                let obj: *mut Object = msg_send![cls, alloc];
                let obj: *mut Object = msg_send![obj, init];
                obj
            };
            msg_send![configuration, setUserContentController:scripthandler];

            let view = NSWindow::contentView(window as id);
            let window_frame = NSView::frame(view as id);
            
            // WKWebView
            let cls = Class::get("WKWebView").ok_or("WKWebView does not exist")?;
            let webview = {
                let obj: *mut Object = msg_send![cls, alloc];
                let obj: *mut Object = msg_send![obj,
                    initWithFrame: window_frame
                    configuration: configuration ];
                obj
            };

            // WKNavigationDelegate
            let cls = navigation_delegate_class();
            let navigation_delegate = {
                let obj: *mut Object = msg_send![cls, alloc];
                let obj: *mut Object = msg_send![obj, init];
                obj
            };
            msg_send![webview, setNavigationDelegate:navigation_delegate];

            NSView::addSubview_(view, webview);
            // NSWindow::addView_(window as id, webview);

            Ok(WebView {
                id: webview
            })
        }
    }

    pub fn load_html_string(&mut self, html: &str) -> Result<(), String> {
        unsafe {
            let cls = Class::get("NSURL").ok_or("NSURL does not exist")?;
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
        Ok(())
    }
}
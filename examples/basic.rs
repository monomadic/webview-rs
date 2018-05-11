#![allow(deprecated)]

extern crate webview;
extern crate winit;

use webview::*;

const CONTENT: &'static str = "
<html>
    <body>
        <button style='width: 150px' onclick=\"window.webkit.messageHandlers.notification.postMessage('hello there');\">
            PRESS ME
        </button>
    </body>
</html>";

fn main() {
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::Window::new(&events_loop).unwrap();
    let msg = "Hello!";

    match run(
        unsafe { window.platform_window() as *mut ::std::os::raw::c_void },
        CONTENT,
        move |webview| { println!("--init {} {:?}", msg, webview.id) },
        move |webview, name, body| { println!("--event {} {} {}", msg, name, body) },
    ){
        Ok(_) => println!("done."),
        Err(e) => println!("error: {}", e),
    }

    events_loop.run_forever(|event| {
        match event {
            winit::Event::WindowEvent {
              event: winit::WindowEvent::CloseRequested,
              ..
            } => winit::ControlFlow::Break,
            _ => winit::ControlFlow::Continue,
        }
    });
}

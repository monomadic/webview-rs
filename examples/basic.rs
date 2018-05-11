#![allow(deprecated)]

extern crate webview;
extern crate winit;

use webview::*;

const CONTENT: &'static str = "
<html>
    <body>
        <button style='width: 150px' onclick=\"window.webkit.messageHandlers.notification.postMessage('event 1');\">event 1</button>
        <button style='width: 150px' onclick=\"window.webkit.messageHandlers.notification.postMessage('event 2');\">event 2</button>
    </body>
</html>";

fn main() {
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::Window::new(&events_loop).unwrap();
    let msg = "Hello!";
    let callback = move |_webview, name, body| { println!("--event {} {} {}", msg, name, body) };

    match run(
        unsafe { window.platform_window() as *mut ::std::os::raw::c_void },
        CONTENT,
        callback,
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

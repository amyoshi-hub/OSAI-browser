use tao::{
  dpi::PhysicalSize,
  event::{Event, StartCause, WindowEvent},
  event_loop::{ControlFlow, EventLoopBuilder},
  window::{CursorIcon, ResizeDirection, Window, WindowBuilder},
};
use wry::{http::Request, WebViewBuilder};

fn main() -> wry::Result<()> {
  let event_loop = EventLoopBuilder::<()>::with_user_event().build();
  let window = WindowBuilder::new()
    .with_decorations(false)
    .build(&event_loop)
    .unwrap();

  let proxy = event_loop.create_proxy();

  let url = "http://google.com"; // URLを指定
  let handler = move |req: Request<String>| {
    let body = req.body();
    println!("Received IPC message: {body}");
    // 必要に応じてイベントを処理
    if body == "close" {
      let _ = proxy.send_event(());
    }
  };

  let builder = WebViewBuilder::new()
    .with_url(url) // URLをロード
    .with_ipc_handler(handler)
    .with_accept_first_mouse(true);

  #[cfg(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
  ))]
  let webview = builder.build(&window)?;
  #[cfg(not(any(
    target_os = "windows",
    target_os = "macos",
    target_os = "ios",
    target_os = "android"
  )))]
  let webview = {
    use tao::platform::unix::WindowExtUnix;
    use wry::WebViewBuilderExtUnix;
    let vbox = window.default_vbox().unwrap();
    builder.build_gtk(vbox)?
  };

  let mut webview = Some(webview);

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("Wry application started!"),
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => {
        let _ = webview.take();
        *control_flow = ControlFlow::Exit;
      }
      Event::UserEvent(_) => {
        let _ = webview.take();
        *control_flow = ControlFlow::Exit;
      }
      _ => (),
    }
  });
}


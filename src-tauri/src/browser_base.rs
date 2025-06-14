use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};
use wry::{
    dpi::{LogicalPosition, LogicalSize},
    Rect, WebViewBuilder,
};
use url::Url;

/// アプリケーションの状態を保持する構造体。
/// ウィンドウと2つのWebViewのハンドルを保持します。
#[derive(Default)]
struct State {
    window: Option<Window>,
    // 上部のコントロール用WebView (index.htmlを読み込む)
    control_webview: Option<wry::WebView>,
    // 下部のコンテンツ表示用WebView
    content_webview: Option<wry::WebView>,
}

impl ApplicationHandler for State {
    /// アプリケーションが再開されたときに呼び出されます。
    /// ここでウィンドウとWebViewを初期化します。
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // メインウィンドウの属性を設定
        let mut attributes = Window::default_attributes();
        // 初期サイズを論理ピクセルで800x800に設定
        attributes.inner_size = Some(LogicalSize::new(800, 800).into());
        // ウィンドウを作成
        let window = event_loop.create_window(attributes).unwrap();

        // ウィンドウの論理サイズとスケールファクターを取得
        let size = window.inner_size().to_logical::<u32>(window.scale_factor());

        // ★高さの計算を変更する変数★
        let control_panel_height = size.height / 6; // 例: 全体の1/6の高さにする
        let content_panel_height = size.height - control_panel_height;
        let content_panel_y_pos = control_panel_height; // 下部パネルのY座標は上部パネルの高さから始まる

        // --- index.html のパスを解決 ---
        let current_exe_path = std::env::current_exe().expect("Failed to get current executable path");
        let project_root = current_exe_path.parent()
                                           .and_then(|p| p.parent())
                                           .and_then(|p| p.parent())
                                           .expect("Failed to get project root directory");

        let index_html_path = project_root.join("index.html");
        let index_html_url = Url::from_file_path(&index_html_path)
            .expect("Failed to create URL from index.html path");


        // --- 上部コントロール用WebViewの作成 ---
        let control_webview = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, 0).into(),
                size: LogicalSize::new(size.width, control_panel_height).into(), // ★高さを変更★
            })
            .with_url(index_html_url.as_str())
            .with_transparent(false)
            .build_as_child(&window)
            .unwrap();

        // --- 下部コンテンツ表示用WebViewの作成 ---
        let content_webview = WebViewBuilder::new()
            .with_bounds(Rect {
                position: LogicalPosition::new(0, content_panel_y_pos).into(), // ★Y座標を変更★
                size: LogicalSize::new(size.width, content_panel_height).into(), // ★高さを変更★
            })
            .with_url("https://www.google.com")
            .with_transparent(false)
            .build_as_child(&window)
            .unwrap();

        // 作成したウィンドウとWebViewをStateに保存
        self.window = Some(window);
        self.control_webview = Some(control_webview);
        self.content_webview = Some(content_webview);
    }

    /// ウィンドウイベントが発生したときに呼び出されます。
    /// 特にリサイズイベントでWebViewのサイズと位置を調整します。
    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) => {
                if let (Some(window), Some(control_webview), Some(content_webview)) =
                    (&self.window, &self.control_webview, &self.content_webview)
                {
                    // ウィンドウの新しい論理サイズを取得
                    let size = size.to_logical::<u32>(window.scale_factor());

                    // ★リサイズ時も高さを変更★
                    let control_panel_height = size.height / 6; // 例: 全体の1/6の高さにする
                    let content_panel_height = size.height - control_panel_height;
                    let content_panel_y_pos = control_panel_height;

                    // 上部コントロール用WebViewのサイズと位置を更新
                    control_webview
                        .set_bounds(Rect {
                            position: LogicalPosition::new(0, 0).into(),
                            size: LogicalSize::new(size.width, control_panel_height).into(),
                        })
                        .unwrap();

                    // 下部コンテンツ表示用WebViewのサイズと位置を更新
                    content_webview
                        .set_bounds(Rect {
                            position: LogicalPosition::new(0, content_panel_y_pos).into(),
                            size: LogicalSize::new(size.width, content_panel_height).into(),
                        })
                        .unwrap();
                }
            }
            // ウィンドウの閉じるリクエストがあった場合にアプリケーションを終了
            WindowEvent::CloseRequested => {
                std::process::exit(0);
            }
            _ => {} // その他のイベントは無視
        }
    }

    /// イベントループがアイドル状態になる直前に呼び出されます。
    /// Linux環境でのGTKイベント処理に使用されます。
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // Linux特有のGTKイベント処理
        #[cfg(any(
            target_os = "linux",
            target_os = "dragonfly",
            target_os = "freebsd",
            target_os = "netbsd",
            target_os = "openbsd",
        ))]
        {
            while gtk::events_pending() {
                gtk::main_iteration_do(false);
            }
        }
    }
}

/// メイン関数
fn main() -> wry::Result<()> {
    // Linux特有の初期化とWaylandチェック
    #[cfg(any(
        target_os = "linux",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
    ))]
    {
        use gtk::prelude::DisplayExtManual;

        gtk::init()?;
        if gtk::gdk::Display::default().unwrap().backend().is_wayland() {
            panic!("This example doesn't support wayland!");
        }

        winit::platform::x11::register_xlib_error_hook(Box::new(|_display, error| {
            let error = error as *mut x11_dl::xlib::XErrorEvent;
            (unsafe { (*error).error_code }) == 170
        }));
    }

    // イベントループを作成し、Stateを渡してアプリケーションを実行
    let event_loop = EventLoop::new().unwrap();
    let mut state = State::default();
    event_loop.run_app(&mut state).unwrap();

    Ok(())
}


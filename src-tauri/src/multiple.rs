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

/// アプリケーションの状態を保持する構造体。
/// ウィンドウと2つのWebViewのハンドルを保持します。
#[derive(Default)]
struct State {
    window: Option<Window>,
    // 上部のコントロール用WebView
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
        // to_logical::<u32>で論理ピクセルに変換しています
        let size = window.inner_size().to_logical::<u32>(window.scale_factor());

        // --- 上部コントロール用WebViewの作成 ---
        let control_webview = WebViewBuilder::new()
            .with_bounds(Rect {
                // 位置はウィンドウの左上 (0, 0)
                position: LogicalPosition::new(0, 0).into(),
                // サイズはウィンドウ幅全体、高さは半分
                size: LogicalSize::new(size.width, size.height / 2).into(),
            })
            // ここにindex.htmlのパスを指定します。
            // Wryがローカルファイルを処理する方法に応じて、適切なURLスキームを使用してください。
            // Tauriのコンテキストでバンドルされたファイルを指す一般的な方法として `tauri://localhost/index.html` を例示します。
            // pure Wryの場合は `file://` スキームを使用するか、カスタムプロトコルハンドラを設定する必要があります。
            .with_url("http://localhost:3000/index.html")
            .build_as_child(&window)
            .unwrap();

        // --- 下部コンテンツ表示用WebViewの作成 ---
        let content_webview = WebViewBuilder::new()
            .with_bounds(Rect {
                // 位置はウィンドウの左下 (Y座標はウィンドウ高さの半分から開始)
                position: LogicalPosition::new(0, size.height / 2).into(),
                // サイズはウィンドウ幅全体、高さは半分
                size: LogicalSize::new(size.width, size.height / 2).into(),
            })
            .with_url("https://www.google.com") // 初期コンテンツURL
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

                    // 上部コントロール用WebViewのサイズと位置を更新
                    control_webview
                        .set_bounds(Rect {
                            position: LogicalPosition::new(0, 0).into(),
                            size: LogicalSize::new(size.width, size.height / 2).into(),
                        })
                        .unwrap();

                    // 下部コンテンツ表示用WebViewのサイズと位置を更新
                    content_webview
                        .set_bounds(Rect {
                            position: LogicalPosition::new(0, size.height / 2).into(),
                            size: LogicalSize::new(size.width, size.height / 2).into(),
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
        // Linux特有のGTKイベント処理 (GTKが提供するX11/Waylandとのインタラクション)
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
        target_os = "netbs",
        target_os = "openbsd",
    ))]
    {
        // GTK関連のインポート
        use gtk::prelude::DisplayExtManual;

        // GTKの初期化
        gtk::init()?;
        // Wayland環境の場合、例示のためにpanicさせています
        if gtk::gdk::Display::default().unwrap().backend().is_wayland() {
            panic!("This example doesn't support wayland!");
        }

        // X11のエラーフックを登録 (X11との低レベルなインタラクション)
        // wryがwinitを通じてX11を使っているため
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


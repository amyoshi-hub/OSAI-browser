use webview::*;

fn main() {
    println!("start web start");
    let handle1 = std::thread::spawn(|| {
        webview::builder()
            .title("Tab 1")
            .content(Content::Html("<h1>Tab 1 Content</h1>"))
            .size(400, 300)
            .run()
            .unwrap();
    });

    let handle2 = std::thread::spawn(|| {
        webview::builder()
            .title("Tab 2")
            .content(Content::Html("<h1>Tab 2 Content</h1>"))
            .size(400, 300)
            .run()
            .unwrap();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}


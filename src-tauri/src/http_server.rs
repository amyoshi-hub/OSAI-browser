use std::convert::Infallible;
use warp::Filter;
use tauri::AppHandle;
use tauri::{WebviewUrl};
use tauri::webview::WebviewWindowBuilder;
use reqwest;
use serde::Deserialize;
//use tower_http::follow_redirect::policy::PolicyExt;

async fn hello(name: String) -> Result<impl warp::Reply, Infallible> {
    Ok(format!("hello {}!", name))
}

#[tauri::command]
pub async fn http_server() {
    let dir = "/home/amyoshi9/git/OSAI-browser/src-tauri/target/debug/share";

    let files = warp::fs::dir(dir);

    println!("Starting HTTP file server at http://127.0.0.1:1234/");
    warp::serve(files)
        .run(([127, 0, 0, 1], 1234))
        .await;
}

#[tauri::command]
pub fn open_url_window(app_handle: AppHandle, url: String) -> Result<(), String> {
    WebviewWindowBuilder::new(&app_handle, "new_window", WebviewUrl::External(url.parse().unwrap()))
        .title("New Window")
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct FileList {
    pub files: Vec<String>,
}

#[tauri::command]
pub async fn fetch_file_list(url: String) -> Result<Vec<String>, String> {
    // GETリクエスト
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("リクエスト失敗: {}", e))?;

    // レスポンス本文（文字列）
    let body = resp.text()
        .await
        .map_err(|e| format!("レスポンス読み込み失敗: {}", e))?;

    // JSON形式でファイル名配列をパースする想定
    // 例: {"files": ["a.txt", "b.png", "c.json"]}
    let file_list: FileList = serde_json::from_str(&body)
        .map_err(|e| format!("JSONパース失敗: {}", e))?;

    Ok(file_list.files)
}

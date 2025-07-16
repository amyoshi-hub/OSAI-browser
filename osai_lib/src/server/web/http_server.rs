//use std::convert::Infallible;
//use warp::Filter;
use reqwest;
use serde::Deserialize;
//use std::fs;
//use std::path::Path;
use base64;

pub async fn http_server() {
    let dir = "/share";

    let files = warp::fs::dir(dir);

    println!("Starting HTTP file server at http://127.0.0.1:1234/");
    warp::serve(files)
        .run(([127, 0, 0, 1], 1234))
        .await;
}

#[derive(Debug, Deserialize)]
pub struct FileList {
    pub files: Vec<String>,
}

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

pub async fn request_file(file_name: String, ip: String) -> Result<String, String> {
    let file_url = format!("http://{}:1234/share/{}", ip, file_name);

    // HTTP GET リクエストを送信
    let response = reqwest::get(&file_url)
        .await
        .map_err(|e| format!("HTTP リクエスト失敗: {}", e))?;

    // ステータスコードを確認
    if !response.status().is_success() {
        return Err(format!("HTTP エラー: {}", response.status()));
    }

    // バイナリデータを取得
    //let bytes = response.bytes().await.map(|bytes| bytes.to_vec()).map_err(|e| format!("レスポンス読み込み失敗: {}", e));
    let bytes = response.bytes().await.map_err(|e| format!("レスポンス読み込み失敗: {}", e))?;
    let base64_encoded = base64::encode(&bytes);
    Ok(base64_encoded)
}


import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import "./App.css";

const worldSearch: React.FC = () => {
  const [downloadUrl, setDownloadUrl] = useState("");
  const [downloadFileName, setDownloadFileName] = useState("downloaded_file");
  const [downloadMessage, setDownloadMessage] = useState("");

  const handleDownload = async () => {
    setDownloadMessage("ダウンロード中...");
    try {
      const result = await invoke("download_file", { // Rustのコマンドを呼び出す
        url: downloadUrl,
        fileName: downloadFileName,
      });
      setDownloadMessage(result as string); // Rustからの成功メッセージ
    } catch (error) {
      setDownloadMessage(`ダウンロードエラー: ${error}`); // Rustからのエラーメッセージ
    }
  };

  return (
    <div>
    <a href="index.html">戻る</a> 

      <p><h2>ファイルダウンロード</h2></p>
      <input
        type="text"
        value={downloadUrl}
        onChange={(e) => setDownloadUrl(e.target.value)}
        placeholder="ダウンロードURLを入力 (例: https://example.com/file.wasm)"
      />
      <input
        type="text"
        value={downloadFileName}
        onChange={(e) => setDownloadFileName(e.target.value)}
        placeholder="保存ファイル名 (例: my_wasm_app.wasm)"
      />
      <button onClick={handleDownload}>ダウンロード開始</button>
      <p>{downloadMessage}</p>
    </div>
  );
};

export default worldSearch;

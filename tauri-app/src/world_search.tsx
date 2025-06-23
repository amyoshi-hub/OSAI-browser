import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import "./App.css";

const worldSearch: React.FC = () => {
  const [downloadUrl, setDownloadUrl] = useState("");
  const [downloadFileName, setDownloadFileName] = useState("downloaded_file");
  const [downloadMessage, setDownloadMessage] = useState("");

  const handleDownload = async () => {
    setDownloadMessage("Downloading...");
    try {
      const result = await invoke("download_file", {
        url: downloadUrl,
        fileName: downloadFileName,
      });
      setDownloadMessage(result as string);
    } catch (error) {
      setDownloadMessage(`Download Error: ${error}`); 
    }
  };

  return (
    <div>
    <a href="index.html">戻る</a> 

      <p><h2>File Download</h2></p>
      <input
        type="text"
        value={downloadUrl}
        onChange={(e) => setDownloadUrl(e.target.value)}
        placeholder="Input Download URL"
      />
      <input
        type="text"
        value={downloadFileName}
        onChange={(e) => setDownloadFileName(e.target.value)}
        placeholder="Save File Name"
      />
      <button onClick={handleDownload}>Download Start</button>
      <p>{downloadMessage}</p>
    </div>
  );
};

export default worldSearch;

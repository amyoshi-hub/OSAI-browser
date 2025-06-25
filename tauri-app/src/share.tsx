import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

const Share: React.FC = () => {
  // Stateでfilesを管理
  const [files, setFiles] = useState<string[]>([]);

  const openNewWindow = async () => {
    try {
      const serverUrl: string = await invoke("http_server");
      console.log("Server started at:", serverUrl);
      await invoke("open_url_window", { url: "http://localhost:1234" });
    } catch (e) {
      console.error("Failed to open new window:", e);
    }
  };

  const loadFileList = async () => {
    try {
      console.log("json parse");
      const fetchedFiles: string[] = await invoke("fetch_file_list", { url: "http://localhost:1234/files.json" });
      console.log("取得したファイルリスト:", fetchedFiles);
      setFiles(fetchedFiles); // Stateにセット
    } catch (error) {
      console.error("取得失敗", error);
    }
  };

  const requestFile = async (fileName: string) => {
  try {
    console.log(`Requesting file: ${fileName}`);
    const fileContent: Uint8Array = await invoke("request_file", { fileName });

    const blob = new Blob([fileContent], { type: "application/octet-stream" });
    const url = URL.createObjectURL(blob);

    const a = document.createElement("a");
    a.href = url;
    a.download = fileName;
    a.click();

    URL.revokeObjectURL(url);
    console.log(`File ${fileName} downloaded successfully.`);
  } catch (error) {
    console.error(`Failed to download file: ${fileName}`, error);
  }
  };
  
  return (
    <div>
      <h1>HTTP File Server</h1>
      <button onClick={openNewWindow}>Start Server and Open</button>
      <button onClick={loadFileList}>Load File List</button>
      <div>
        {files.map((file) => (
          <button key={file} onClick={() => requestFile(file)}>
            {file}
          </button>
        ))}
      </div>
    </div>
  );
};

export default Share;


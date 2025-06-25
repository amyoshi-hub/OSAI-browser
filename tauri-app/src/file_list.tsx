import React, { useState } from "react";
import { invoke } from "@tauri-apps/api/core"; // Tauri API
import "./App.css";

const FileDownloaderPage: React.FC = () => {
    const [fileList, setFileList] = useState<string[]>([]); // ファイルリスト
    const [statusMessage, setStatusMessage] = useState<string>("Click the button to load files.");
    const [errorMessage, setErrorMessage] = useState<string | null>(null);

    // ファイルリストを取得
    const loadFiles = async () => {
        setStatusMessage("Loading files...");
        setErrorMessage(null);

        try {
            const files: string[] = await invoke("get_file_list"); // Rust側コマンド
            setFileList(files);
            setStatusMessage("Files loaded successfully.");
        } catch (error) {
            console.error("Failed to load files:", error);
            setErrorMessage("Failed to load files from the directory.");
            setStatusMessage("Loading files failed.");
        }
    };

    // ファイルを読み取り、ダウンロードリンクを作成
    const downloadFile = async (filePath: string) => {
        setStatusMessage(`Downloading ${filePath}...`);
        try {
            // Rust側からファイルの内容を取得
            const fileContent: string = await invoke("read_file_content", { filePath });

            // Blob を作成し、ダウンロードリンクを生成
            const blob = new Blob([fileContent], { type: "text/plain" }); // 必要に応じて MIME タイプを変更
            const url = URL.createObjectURL(blob);
            const a = document.createElement("a");
            a.href = url;
            a.download = filePath.split("/").pop() || "downloaded_file";
            a.click();
            URL.revokeObjectURL(url);

            setStatusMessage(`${filePath} downloaded successfully.`);
        } catch (error) {
            console.error("Failed to download file:", error);
            setErrorMessage(`Failed to download ${filePath}.`);
        }
    };

    return (
        <div
            style={{
                minHeight: "300px",
                width: "80%",
                margin: "0 auto",
                backgroundColor: "lightgray",
                padding: "20px",
                boxSizing: "border-box",
            }}
        >
            <h2>File Downloader</h2>
            <p>Status: {statusMessage}</p>
            {errorMessage && <p style={{ color: "red" }}>{errorMessage}</p>}

            <div>
                <button onClick={loadFiles} style={{ padding: "10px 20px", marginBottom: "20px" }}>
                    Load Files
                </button>
            </div>

            {fileList.length > 0 ? (
                <ul>
                    {fileList.map((file, index) => (
                        <li key={index}>
                            <button
                                onClick={() => downloadFile(file)}
                                style={{ background: "none", border: "none", color: "blue", textDecoration: "underline", cursor: "pointer" }}
                            >
                                {file.split('/').pop()}
                            </button>
                        </li>
                    ))}
                </ul>
            ) : (
                <p>No files to display yet.</p>
            )}
        </div>
    );
};

export default FileDownloaderPage;


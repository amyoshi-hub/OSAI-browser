import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

const P2P = () => {
  const [ip, setIp] = useState("127.0.0.1");
  const [port, setPort] = useState("1234");
  const [toIp, setToIp] = useState("127.0.0.1");
  const [toPort, setToPort] = useState("1234");
  const [filename, setFilename] = useState("example_file");
  const [viewText, setViewText] = useState("bulletin board");
  const [serverList, setServerList] = useState<string[]>([]);

  const startServer = async () => {
    try {
      console.log(`Starting server at ${ip}:${port}`);
      const result = await invoke<string>("start_server", { ip, port });
      setViewText(result || "Server started successfully.");
    } catch (error) {
      console.error("Server start error:", error);
      setViewText(`Server start failed: ${error}`);
    }
  };

  const sendFile = async () => {
  try {
    console.log(`Sending data from ${ip}:${port} to ${toIp}:${toPort}`);
    const result = await invoke<string>("send_file", {
      src_ip: ip,
      src_port: parseInt(port),
      dst_ip: toIp,
      dst_port: parseInt(toPort),
      filename,
    });
    setViewText(result || "File sent successfully.");
  } catch (error) {
    console.error("File send error:", error);
    setViewText(`File send failed: ${error}`);
  }
  };
 
  const loadText = async () => {
    try {
      const text = await invoke<string>("rust_code");
      setViewText(text);
      console.log("Message from Rust:", text);
    } catch (error) {
      console.error("Error loading text from Rust:", error);
    }
  };

  const addServer = (server: string) => {
    setServerList((prev) => [...prev, server]);
  };

  return (
    <div>
      <a href="index.html">Back to Menu</a>
      <h2>Setup as Server</h2>
      <div>
        <label>
          IP: 
          <input value={ip} onChange={(e) => setIp(e.target.value)} placeholder="127.0.0.1" />
        </label>
        <label>
          Port:
          <input value={port} onChange={(e) => setPort(e.target.value)} placeholder="1234" />
        </label>
      </div>
      <button onClick={startServer}>Start Server</button>
      
      <h2>Server List</h2>
      <div>
        {serverList.length === 0 ? (
          <p>No servers available</p>
        ) : (
          <ul>
            {serverList.map((server, index) => (
              <li key={index}>{server}</li>
            ))}
          </ul>
        )}
      </div>

      <h2>P2P Channel</h2>
      <div>
        <button onClick={loadText}>Load Bulletin Board</button>
        <p>{viewText}</p>
        <button onClick={() => addServer(`${ip}:${port}`)}>Add Server</button>
      </div>

      <h2>Client</h2>
      <div>
        <label>
          From IP: 
          <input value={ip} onChange={(e) => setIp(e.target.value)} placeholder="127.0.0.1" />
        </label>
        <label>
          From Port:
          <input value={port} onChange={(e) => setPort(e.target.value)} placeholder="1234" />
        </label>
        <label>
          To IP:
          <input value={toIp} onChange={(e) => setToIp(e.target.value)} placeholder="127.0.0.1" />
        </label>
        <label>
          To Port:
          <input value={toPort} onChange={(e) => setToPort(e.target.value)} placeholder="1234" />
        </label>
        <label>
          File Name:
          <input value={filename} onChange={(e) => setFilename(e.target.value)} placeholder="example_file" />
        </label>
      </div>
      <button onClick={sendFile}>Send File</button>
    </div>
  );
};

export default P2P;


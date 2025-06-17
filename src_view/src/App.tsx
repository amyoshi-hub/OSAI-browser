import { useState, useEffect } from "react"; // Add useState and useEffect
import { emit } from "@tauri-apps/api/event"; // For the event emit button
import { invoke } from "@tauri-apps/api";

// Frontend component for Tauri app
function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState(""); // Also declare name state

  // Function to call the Rust command
  async function greet() {
    // Rustのgreetコマンドが削除されているため、ここでは `rust_code` を呼び出します
    // Rust側が `greet` コマンドを受け付ける場合は、`invoke("greet", { name })` に戻せます
    // 今は `rust_code` のみが Rust 側にあるため、これを使用
    // name は現在 Rust 側で受け取っていないので、空オブジェクトを渡します
    setGreetMsg(await invoke("rust_code", {})); 
  }

  // Effect to handle the "Click to emit message" button (if it emits a Rust command, not just frontend event)
  // The original App.tsx had a button that called `greet()`.
  // If the intent of "Click to emit message" is to emit a frontend event,
  // we should use the emit function from `@tauri-apps/api/event`.
  // Let's assume you want to emit a frontend-to-backend event for now.
  const handleEmitMessage = () => {
    emit('front-to-back', 'hello from front - from button'); // Emitting the event
  };


  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          {/* Corrected path for React logo */}
          <img src="/react.svg" className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <div className="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            greet(); // Call greet function
          }}
        >
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)} // setName is now defined
            placeholder="Enter a name..."
          />
          <button type="submit">Greet</button>
        </form>
      </div>
      <p>{greetMsg}</p> {/* greetMsg is now defined */}

      {/* Button to emit a frontend event to the backend */}
      {/* If this button is meant to call the Rust 'greet' command, keep `onClick={greet}`.
          If it's meant to emit a frontend event (like "front-to-back"), use handleEmitMessage.
          I'll assume 'front-to-back' event for now as it aligns with your Rust listener.
      */}
      <div>Hello Tauri</div>
      <button onClick={handleEmitMessage}>Click to emit message (Frontend Event)</button>
    </div>
  );
}

export default App;

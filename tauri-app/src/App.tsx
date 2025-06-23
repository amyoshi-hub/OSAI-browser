import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import "./App.css";

const App: React.FC = () => {
  const [url, setUrl] = useState("");
  const [useIframe, setUseIframe] = useState(true);
  const [iframeSrc, setIframeSrc] = useState("");
  const [instructionText, setInstructionText] = useState("Iframe MODE Please input URL");

  const navigate = useNavigate();

  const updateContent = (newUrl: string) => {
    if (useIframe) {
      setIframeSrc(newUrl);
    } else {
      window.location.href = newUrl;
    }
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    setUrl(e.target.value);
  };

  const handleInputKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter") {
      updateContent(url);
    }
  };

  const switchMode = () => {
    const newMode = !useIframe;
    setUseIframe(newMode);
    setInstructionText(`NOW: ${newMode ? "Iframe MODE" : "NO Iframe MODE"}`);
  };

  const loadP2PPage = () => {
	  navigate("/p2p");
  };

  const loadWorldPage = () => {
	  navigate("/world_page");
  };

  return (
    <div>
      <div className="address-bar">
        <input
          type="text"
          value={url}
          onChange={handleInputChange}
          onKeyDown={handleInputKeyDown}
          placeholder="INPUT URL"
        />
        <button onClick={switchMode}>Switch Mode</button>
        <button onClick={loadP2PPage}>P2P</button>
        <button onClick={loadWorldPage}>WORLD_SELECT</button>
      </div>
      <p id="instruct_text">{instructionText}</p>
      {useIframe && (
        <iframe id="mainIframe" src={iframeSrc} style={{ width: "100%", height: "calc(100vh - 50px)", border: "none" }} />
      )}
    </div>
  );
};

export default App;


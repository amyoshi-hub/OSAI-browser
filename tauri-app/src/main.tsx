import React from "react";
import ReactDOM from "react-dom/client";
import { HashRouter, Routes, Route } from "react-router-dom";
import App from "./App";
import P2P from "./p2p";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
  <HashRouter>
  <Routes>
    <Route path="/" element={<App />} />
    <Route path="/p2p" element={<P2P />} />
  </Routes>
  </HashRouter>
  </React.StrictMode>,
);

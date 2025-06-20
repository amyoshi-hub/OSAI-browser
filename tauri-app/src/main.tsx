import React from "react";
import ReactDOM from "react-dom/client";
import { HashRouter, Routes, Route } from "react-router-dom";
import App from "./App";
import P2P from "./p2p";
import WORLD_PAGE from "./world_page";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
  <HashRouter>
  <Routes>
    <Route path="/" element={<App />} />
    <Route path="/p2p" element={<P2P />} />
    <Route path="/world_page" element={<WORLD_PAGE />} />
  </Routes>
  </HashRouter>
  </React.StrictMode>,
);

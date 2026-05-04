import React from "react";
import ReactDOM from "react-dom/client";
import { HashRouter, Route, Routes } from "react-router-dom";
import "./index.css";
import PopoverPage from "./routes/popover/Page";
import SettingsPage from "./routes/settings/Page";
import { bindWallpaperEvents } from "./store/wallpaper";
import { bindSettingsEvents } from "./store/settings";

bindWallpaperEvents();
bindSettingsEvents();

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <HashRouter>
      <Routes>
        <Route path="/" element={<PopoverPage />} />
        <Route path="/settings" element={<SettingsPage />} />
      </Routes>
    </HashRouter>
  </React.StrictMode>,
);

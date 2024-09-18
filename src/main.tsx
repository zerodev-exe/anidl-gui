import React from "react";
import ReactDOM from "react-dom/client";
import Header from "../components/Header";
import { DownloadProvider } from '../contexts/DownloadContext';

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <DownloadProvider>
      <Header />
    </DownloadProvider>
  </React.StrictMode>,
);
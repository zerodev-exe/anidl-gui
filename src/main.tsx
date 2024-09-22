import React from "react";
import ReactDOM from "react-dom/client";
import Header from "../components/Header";
import { DownloadProvider } from '../contexts/DownloadContext';
import { SearchProvider } from '../contexts/SearchContext';

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <SearchProvider>
      <DownloadProvider>
        <Header />
      </DownloadProvider>
    </SearchProvider>
  </React.StrictMode>,
);
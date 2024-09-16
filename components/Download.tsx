import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./Download.css";

function Download() {
  const [downloading, setDownloading] = useState<string[]>([]);
  const [downloaded, setDownloaded] = useState<string[]>([]);

  useEffect(() => {
    const checkDownloads = async () => {
      try {
        const result = await invoke("check_downloads");
        const { downloading: downloadingList, downloaded: downloadedList } = result as { downloading: string[], downloaded: string[] };
        setDownloading(downloadingList);
        setDownloaded(downloadedList);
      } catch (error) {
        console.error("Error checking downloads:", error);
      }
    };

    checkDownloads();
    const interval = setInterval(checkDownloads, 500); // Check every 500 milliseconds (0.5 seconds)

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="download-container">
      <h1>Downloaded</h1>
      <ul>
        {downloaded.map((folder, index) => (
          <li key={index}>{folder}</li>
        ))}
      </ul>
      <h1>Downloading...</h1>
      <ul>
        {downloading.map((folder, index) => (
          <li key={index}>{folder}</li>
        ))}
      </ul>
    </div>
  );
}

export default Download;
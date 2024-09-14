import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

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
    const interval = setInterval(checkDownloads, 5000); // Check every 5 seconds

    return () => clearInterval(interval);
  }, []);

  return (
    <div>
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
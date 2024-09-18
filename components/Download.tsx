import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useDownload, DownloadInfo } from '../contexts/DownloadContext';
import "./Download.css";
import DownloadProgress from "./DownloadProgress";

function Download() {
    const { downloading, setDownloading, downloaded, setDownloaded } = useDownload();

    useEffect(() => {
        const checkDownloads = async () => {
            try {
                const result = await invoke("check_downloads");
                const { downloading: downloadingList, downloaded: downloadedList, progress: progressList } = result as { downloading: string[], downloaded: string[], progress: { [key: string]: [number, number] } };
                
                const createDownloadInfo = (folder: string): DownloadInfo => {
                    const [downloadedEpisodes, totalEpisodes] = progressList[folder] || [0, 0];
                    const progress = totalEpisodes > 0 ? (downloadedEpisodes / totalEpisodes) * 100 : 0;
                    return {
                        folder,
                        progress,
                        downloadedEpisodes,
                        totalEpisodes
                    };
                };

                setDownloading(downloadingList.map(createDownloadInfo));
                setDownloaded(downloadedList.map(createDownloadInfo));
            } catch (error) {
                console.error("Error checking downloads:", error);
            }
        };
        checkDownloads();
        const interval = setInterval(checkDownloads, 500);
        return () => clearInterval(interval);
    }, [setDownloading, setDownloaded]);

    return (
        <div className="download-container">
            <h1>Downloading...</h1>
            <div className="download-grid">
                {downloading.map((info, index) => (
                    <div key={index} className="download-item">
                        <span className="folder-name">{info.folder}</span>
                        <DownloadProgress 
                            progress={info.progress} 
                            fileName={info.folder} 
                            downloadedEpisodes={info.downloadedEpisodes}
                            totalEpisodes={info.totalEpisodes}
                        />
                    </div>
                ))}
            </div>
            <h1>Downloaded</h1>
            <div className="download-grid">
                {downloaded.map((info, index) => (
                    <div key={index} className="download-item">
                        <span className="folder-name">{info.folder}</span>
                        <DownloadProgress 
                            progress={info.progress} 
                            fileName={info.folder} 
                            downloadedEpisodes={info.downloadedEpisodes}
                            totalEpisodes={info.totalEpisodes}
                        />
                    </div>
                ))}
            </div>
        </div>
    );
}

export default Download;
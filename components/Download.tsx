import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useDownload, DownloadInfo } from '../contexts/DownloadContext';
import "./Download.css";
import DownloadProgress from "./DownloadProgress";

function Download() {
    const { downloading, setDownloading, downloaded, setDownloaded, ongoing, setOngoing } = useDownload();

    useEffect(() => {
        const checkDownloads = async () => {
            try {
                const result = await invoke("check_downloads");
                const { downloading: downloadingList, downloaded: downloadedList, ongoing: ongoingList, progress: progressList } = result as { downloading: string[], downloaded: string[], ongoing: string[], progress: { [key: string]: [number, number] } };
                
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
                setOngoing(ongoingList.map(createDownloadInfo)); // Set ongoing downloads
            } catch (error) {
                console.error("Error checking downloads:", error);
            }
        };
        checkDownloads();
        const interval = setInterval(checkDownloads, 5000);
        return () => clearInterval(interval);
    }, [setDownloading, setDownloaded, setOngoing]);

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

            <h1>Ongoing</h1>
            <div className="download-grid">
                {ongoing.map((info, index) => (
                    <div key={index} className="download-item">
                        <span className="folder-name">{info.folder}</span>
                        <DownloadProgress 
                            progress={info.progress} 
                            fileName={info.folder} 
                            downloadedEpisodes={info.downloadedEpisodes}
                            totalEpisodes={info.totalEpisodes}
                            color="#00008b" // Set the color to blue for ongoing downloads
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
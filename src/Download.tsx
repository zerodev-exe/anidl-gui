import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useDownload, DownloadInfo } from '../contexts/DownloadContext';
import "./Download.css";
import DownloadProgress from "../components/DownloadProgress";

interface SectionProps {
    title: string;
    items: { folder: string; progress: number; downloadedEpisodes: number; totalEpisodes: number }[];
    color: string;
}

function Download() {
    const { downloading, setDownloading, downloaded, setDownloaded, ongoing, setOngoing } = useDownload();
    const [loading, setLoading] = useState(true); // Initial loading state

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

                // Batch state updates
                setDownloading(downloadingList.map(createDownloadInfo));
                setDownloaded(downloadedList.map(createDownloadInfo));
                setOngoing(ongoingList.map(createDownloadInfo));
            } catch (error) {
                console.error("Error checking downloads:", error);
            } finally {
                setLoading(false); // Set loading to false when done
            }
        };

        checkDownloads();
        const interval = setInterval(checkDownloads, 5000);
        return () => clearInterval(interval);
    }, [setDownloading, setDownloaded, setOngoing]);

    const handleDownloadClick = async (folder: string) => {
        if (!folder) {
            console.error("Anime name is empty. Cannot start download.");
            return;
        }
        try {
            await invoke("retry_button", { animeName: folder });
            console.log(`Started downloading: ${folder}`);
        } catch (error) {
            console.error("Error starting download:", error);
        }
    };

    return (
        <div className="download-container">
            {loading ? ( // Display loading indicator only at the beginning
                <h1>Loading...</h1>
            ) : (
                <>
                    <h1>Downloading...</h1>
                    <div className="download-grid">
                        {downloading.map((info, index) => (
                            <div key={index} className="download-item">
                                <h3 className="folder-name">{info.folder}</h3>
                                <center>
                                    <button onClick={() => handleDownloadClick(info.folder)}>
                                        Retry downloading {info.folder}
                                    </button>
                                </center>
                                <DownloadProgress 
                                    progress={info.progress} 
                                    fileName={info.folder} 
                                    downloadedEpisodes={info.downloadedEpisodes}
                                    totalEpisodes={info.totalEpisodes}
                                />
                            </div>
                        ))}
                    </div>

                    <Section title="Ongoing" items={ongoing} color="#00008b" />
                    <Section title="Downloaded" items={downloaded} color="#222222" />
                </>
            )}
        </div>
    );
}

function Section({ title, items, color }: SectionProps) {
    return (
        <>
            <h1>{title}</h1>
            <div className="download-grid">
                {items.map((info, index) => (
                    <div key={index} className="download-item">
                        <span className="folder-name">{info.folder}</span>
                        <DownloadProgress 
                            progress={info.progress} 
                            fileName={info.folder} 
                            downloadedEpisodes={info.downloadedEpisodes}
                            totalEpisodes={info.totalEpisodes}
                            color={color}
                        />
                    </div>
                ))}
            </div>
        </>
    );
}

export default Download;
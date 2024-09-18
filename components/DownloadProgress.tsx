import React from "react";
import "./DownloadProgress.css";

interface DownloadProgressProps {
    progress: number;
    fileName: string;
    downloadedEpisodes: number;
    totalEpisodes: number;
}

const DownloadProgress: React.FC<DownloadProgressProps> = ({ progress, fileName, downloadedEpisodes, totalEpisodes }) => {
    const displayProgress = isNaN(progress) ? 0 : progress;
    return (
        <div className="download-progress">
            <div className="progress-bar" style={{ width: `${displayProgress}%` }}></div>
            <span id="progress-text">
                {fileName}: {downloadedEpisodes}/{totalEpisodes} episodes ({displayProgress.toFixed(1)}%)
            </span>
        </div>
    );
};

export default DownloadProgress;

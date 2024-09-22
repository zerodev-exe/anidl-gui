import React from "react";
import "./DownloadProgress.css";

interface DownloadProgressProps {
    progress: number;
    fileName: string;
    downloadedEpisodes: number;
    totalEpisodes: number;
    color?: string; // New optional color prop
}

const DownloadProgress: React.FC<DownloadProgressProps> = ({ progress, fileName, downloadedEpisodes, totalEpisodes, color }) => {
    const displayProgress = isNaN(progress) ? 0 : progress;
    return (
        <div className="download-progress">
            <div className="progress-bar" style={{ width: `${displayProgress}%`, backgroundColor: color || '#006400' }}></div>
            <span className="progress-text">
                {fileName}: {downloadedEpisodes}/{totalEpisodes} episodes ({displayProgress.toFixed(1)}%)
            </span>
        </div>
    );
};

export default DownloadProgress;

import React, { createContext, useContext, useState } from 'react';

export interface DownloadInfo {
    folder: string;
    progress: number;
    downloadedEpisodes: number;
    totalEpisodes: number;
}

interface DownloadContextType {
    downloading: DownloadInfo[];
    downloaded: DownloadInfo[];
    setDownloading: React.Dispatch<React.SetStateAction<DownloadInfo[]>>;
    setDownloaded: React.Dispatch<React.SetStateAction<DownloadInfo[]>>;
}

const DownloadContext = createContext<DownloadContextType | undefined>(undefined);

export const DownloadProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const [downloading, setDownloading] = useState<DownloadInfo[]>([]);
    const [downloaded, setDownloaded] = useState<DownloadInfo[]>([]);

    return (
        <DownloadContext.Provider value={{ downloading, downloaded, setDownloading, setDownloaded }}>
            {children}
        </DownloadContext.Provider>
    );
};

export const useDownload = () => {
    const context = useContext(DownloadContext);
    if (!context) {
        throw new Error('useDownload must be used within a DownloadProvider');
    }
    return context;
};

import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

interface Anime {
  img_url: string;
  title: string;
  url: string;
}

function App() {
  const [searchTerm, setSearchTerm] = useState('');
  const [animeList, setAnimeList] = useState<Anime[]>([]);

  const handleSearch = async () => {
    const result = await invoke('search_anime', { name: searchTerm });
    setAnimeList(result as any);
  };

  const handleDownload = async (url: string) => {
    await invoke('download_anime', { url });
  };

  return (
    <div className="container">
      <h1>Zero's Anime Downloader</h1>
      <form onSubmit={(e) => { e.preventDefault(); handleSearch(); }}>
        <input
          type="text"
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
        />
        <button type="submit">Search</button>
      </form>
      {animeList.map((anime, index) => (
        <div key={index}>
          <img className="thumbnail" src={anime.img_url} alt={anime.title} />
          <p>Title: {anime.title}</p>
          <p>URL: {anime.url}</p>
          <button onClick={() => handleDownload(anime.url)}>Download</button>
        </div>
      ))}
    </div>
  );
}

export default App;

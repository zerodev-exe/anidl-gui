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

  const handleDownload = async (animeUrlEnding: string, animeName: string) => { // Updated key name
    console.log(`Downloading anime with URL ending: ${animeUrlEnding} and name: ${animeName}`);
    try {
      await invoke('download_anime', { animeUrlEnding, animeName }); // Updated key name
    } catch (error) {
      console.error('Error invoking download_anime:', error);
    }
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
        <div className="anime-card" key={index}>
          <img className="thumbnail" src={anime.img_url} alt={anime.title} />
          <p>Title: {anime.title}</p>
          <button onClick={() => {
            console.log(`Button clicked for URL: ${anime.url}`);
            handleDownload(anime.url, anime.title);
          }}>Download</button>
        </div>
      ))}
    </div>
  );
}

export default App;

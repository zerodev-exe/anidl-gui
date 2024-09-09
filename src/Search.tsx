import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

interface Anime {
  img_url: string;
  title: string;
  url: string;
  loading?: boolean;
  status?: "idle" | "downloading" | "downloaded";
}

function Search() {
  const [searchTerm, setSearchTerm] = useState("");
  const [animeList, setAnimeList] = useState<Anime[]>([]);
  const [loading, setLoading] = useState(false);
  const [filterDub, setFilterDub] = useState(false); // Add this line

  const handleSearch = async () => {
    setLoading(true);
    const result = await invoke("search_anime", { name: searchTerm });
    let filteredResult = result as Anime[];
    if (filterDub) {
      filteredResult = filteredResult.filter(anime => anime.title.includes("(Dub)"));
    }
    setAnimeList(filteredResult);
    setLoading(false);
  };

  const handleDownload = async (
    animeUrlEnding: string,
    animeName: string,
    index: number
  ) => {
    console.log(
      `Downloading anime with URL ending: ${animeUrlEnding} and name: ${animeName}`
    );
    try {
      setAnimeList((prevList) => {
        const newList = [...prevList];
        newList[index].loading = true;
        newList[index].status = "downloading";
        return newList;
      });
      await invoke("download_anime", { animeUrlEnding, animeName });
      setAnimeList((prevList) => {
        const newList = [...prevList];
        newList[index].status = "downloaded";
        return newList;
      });
    } catch (error) {
      console.error("Error invoking download_anime:", error);
    } finally {
      setAnimeList((prevList) => {
        const newList = [...prevList];
        newList[index].loading = false;
        return newList;
      });
    }
  };

  return (
    <div>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          handleSearch();
        }}
      >
        <input
          type="text"
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
        />
        <button type="submit" disabled={loading}>
          {loading ? "Searching..." : "Search"}
        </button>
        <label>
          <input
            type="checkbox"
            checked={filterDub}
            onChange={(e) => setFilterDub(e.target.checked)}
          />
          Only show Dubbed
        </label>
      </form>
      {animeList.map((anime, index) => (
        <div className="anime-card" key={index}>
          <img className="thumbnail" src={anime.img_url} alt={anime.title} />
          <p>Title: {anime.title}</p>
          {anime.status === "downloading" || anime.status === "downloaded" ? (
            <h5>
              {anime.status === "downloading" ? "Downloading..." : "Downloaded"}
            </h5>
          ) : (
            <button
              onClick={() => {
                console.log(`Button clicked for URL: ${anime.url}`);
                handleDownload(anime.url, anime.title, index);
              }}
            >
              Download
            </button>
          )}
        </div>
      ))}
    </div>
  );
}

export default Search;

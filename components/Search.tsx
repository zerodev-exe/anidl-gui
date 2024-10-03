import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useSearch } from "../contexts/SearchContext";
import './Search.css'

interface Anime {
  img_url: string;
  title: string;
  url: string;
  loading?: boolean;
  status?: "idle" | "downloading" | "downloaded";
}

function Search() {
  const {
    searchTerm,
    setSearchTerm,
    filterDub,
    setFilterDub,
    filterSub,
    setFilterSub,
  } = useSearch();
  const [loading, setLoading] = useState(false);
  const [animeList, setAnimeList] = useState<Anime[]>([]);

  useEffect(() => {
    const fetchFilters = async () => {
      const isDub = (await invoke("get_filter_dub")) as boolean;
      const isSub = (await invoke("get_filter_sub")) as boolean;
      setFilterDub(isDub);
      setFilterSub(isSub);
    };
    fetchFilters();
  }, []);

  const handleSearch = async () => {
    setLoading(true);
    const result = await invoke("search_anime", { name: searchTerm });
    const filter_word = "(Dub)";
    let filteredResult = result as Anime[];

    if (filterDub) {
      filteredResult = filteredResult.filter((anime) =>
        anime.title.includes(filter_word)
      );
    } else if (filterSub) {
      filteredResult = filteredResult.filter(
        (anime) => !anime.title.includes(filter_word)
      );
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

  const handleCheckboxChangeDub = async (
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    const isChecked = e.target.checked;
    setFilterDub(isChecked);
    if (isChecked) {
      setFilterSub(false); // Uncheck the subbed filter if dubbed is checked
    }
    await invoke("set_filter_dub", { isDub: isChecked });
    await invoke("set_filter_sub", { isSub: !isChecked }); // Update sub filter state
  };

  const handleCheckboxChangeSub = async (
    e: React.ChangeEvent<HTMLInputElement>
  ) => {
    const isChecked = e.target.checked;
    setFilterSub(isChecked);
    if (isChecked) {
      setFilterDub(false); // Uncheck the dubbed filter if subbed is checked
    }
    await invoke("set_filter_sub", { isSub: isChecked });
    await invoke("set_filter_dub", { isDub: !isChecked }); // Update dub filter state
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
          onChange={(e) => {
            setSearchTerm(e.target.value);
            handleSearch();
          }}
        />
        <button type="submit" disabled={loading}>
          {loading ? "Searching..." : "Search"}
        </button>
        <br />
        <label>
          <input
            type="checkbox"
            checked={filterDub}
            onChange={handleCheckboxChangeDub}
          />
          Only show Dubbed
        </label>
        <br />
        <label>
          <input
            type="checkbox"
            checked={filterSub}
            onChange={handleCheckboxChangeSub}
          />
          Only show Subbed
        </label>
      </form>

      <div className="anime">
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

    </div>
  );
}

export default Search;

import Search from './Search';
import z from '/z.svg';

function App() {
	return (
		<div className="container">
			<img className='logo vite' src={z} alt="Zero's Anime Downloader" />
			<h1>Zero's Anime Downloader</h1>
			<Search />
		</div>
	);
}

export default App;

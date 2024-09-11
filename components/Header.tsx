import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import App from '../src/App';
import About from '../src/About';
import Download from '../src/Download';
import Settings from '../src/Settings';
import './Header.css';

function Header() {
    return (
      <Router>
        <header className="header">
          <div>
            <nav className="nav">
              <ul className="nav-list">
                {/* Search bar */}
                <li className="nav-item">
                  <Link to="/" className="nav-link">Home</Link>
                </li>
                {/* Download */}
                <li className="nav-item">
                  <Link to="/download" className="nav-link">Download</Link>
                </li>
                {/* Settings */}
                <li className="nav-item">
                  <Link to="/settings" className="nav-link">Settings</Link>
                </li>
                {/* About */}
                <li className="nav-item">
                  <Link to="/about" className="nav-link">About</Link>
                </li>
              </ul>
            </nav>
          </div>
        </header>
        <div>
          <Routes>
            <Route path="/" element={<App />} />
            <Route path="/about" element={<About />} />
            <Route path="/download" element={<Download />} />
            <Route path="/settings" element={<Settings />} />
          </Routes>
        </div>
      </Router>
    )
}

export default Header;
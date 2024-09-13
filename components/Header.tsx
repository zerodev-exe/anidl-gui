import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import App from '../src/App';
import About from './About';
import Download from './Download';
import Settings from './Settings';
import './Header.css';

function Header() {
    return (
      <Router>
        <header className="header">
          <div>
            <nav className="nav">
              <ul className="nav-list">
                <li className="nav-item">
                  <Link to="/" className="nav-link">Home</Link>
                </li>
                <li className="nav-item">
                  <Link to="/download" className="nav-link">Download</Link>
                </li>
                <li className="nav-item">
                  <Link to="/settings" className="nav-link">Settings</Link>
                </li>
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
    );
}

export default Header;
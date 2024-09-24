import { BrowserRouter as Router, Routes, Route, Link } from "react-router-dom";
import App from "../src/App";
import About from "../src/About";
import Download from "../src/Download";
import Settings from "../src/Settings";
import "./Header.css";

function Header() {
  return (
    <Router>
      <header className="header">
        <div>
          <nav className="nav">
            <ul className="nav-list">
              <Link to="/" className="nav-link">
                <li className="nav-item">Home</li>
              </Link>
              <Link to="/download" className="nav-link">
                <li className="nav-item">Download</li>
              </Link>
              <Link to="/settings" className="nav-link">
                <li className="nav-item">Settings</li>
              </Link>
              <Link to="/about" className="nav-link">
                <li className="nav-item">About</li>
              </Link>
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

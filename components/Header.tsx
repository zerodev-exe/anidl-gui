import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import App from '../src/App';
import About from '../src/About';

function Header() {
    return (
      <Router>
        <div className="container">
        <nav>
          <ul>
            <li>
              <Link to="/">Home</Link>
            </li>
            <li>
              <Link to="/about">About</Link>
            </li>
          </ul>
        </nav>
        <Routes>
          <Route path="/" element={<App />} />
          <Route path="/about" element={<About />} />
        </Routes>
      </div>
    </Router>
    )
}

export default Header;
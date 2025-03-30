import { BrowserRouter as Router, Routes, Route } from "react-router-dom";
import MockLoginPage from "./pages/MockLoginPage";
import MapViewer from "./components/MapViewer";

function App(): React.FC {
  return (
    <Router>
      <Routes>
        <Route path="/mock-login" element={<MockLoginPage />} />
        <Route path="/" element={<MapViewer />} />
      </Routes>
    </Router>
  );
}

export default App;

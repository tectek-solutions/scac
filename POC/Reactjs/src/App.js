import React from "react";
import { BrowserRouter as Router, Routes, Route} from "react-router-dom";
import Home from "./Components/Pages/Home";
import Toto from "./Components/Pages/Toto";
import Header from "./Components/Header";

function App() {
  return (
    <Router>
      <Header />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/toto" element={<Toto />} />
      </Routes>
    </Router>
  );
}

export default App;
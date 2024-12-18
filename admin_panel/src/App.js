import './App.css';
import React, { useState } from 'react';
import NavBar from './components/NavBar';
import { BrowserRouter as Router, Route, Routes } from "react-router-dom";
import Users from './components/pages/tables/users';
import Authentications from './components/pages/tables/authentications';


function App() {
  const [isOpen, setIsOpen] = useState(true);
  return (
    <Router>
      <div className="flex w-full">
        <NavBar isOpen={isOpen} setIsOpen={setIsOpen} />
        <div className={`mt-2 ${isOpen ? "ml-10" : "ml-6"} w-full transition-all duration-300`}>
          <Routes>
            <Route path="/tables/users" element={<Users />} />
            <Route path="/tables/authentications" element={<Authentications />} />
          </Routes>
        </div>
      </div>
    </Router>
  );
}

export default App;
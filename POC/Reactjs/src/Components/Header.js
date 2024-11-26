import React, { useEffect } from "react";
import '../App.css';
import logo from '../logo.svg';

export default function Header() {
    const active = (e) => {
        const links = document.querySelectorAll('.nav-middle ul li a');
        links.forEach((link) => {
            link.classList.remove('active');
        });
        e.target.classList.add('active');
    }

    useEffect(() => {
        const links = document.querySelectorAll('.nav-middle ul li a');
        links.forEach((link) => {
            link.addEventListener('click', active);
        });
    });

    return (
        <header>
            <nav className="navbar">
                <div className="nav-start">
                    <img src={logo} className="App-logo" alt="logo" />
                    <h1>Reactjs</h1>
                </div>
                <div className="nav-middle">
                    <ul>
                        <li><a href="/">Home</a></li>
                        <li><a href="/toto">Toto</a></li>
                    </ul>
                </div>
                <div className="nav-end">
                    <h1>POC</h1>
                </div>
            </nav>
        </header>
    );
};
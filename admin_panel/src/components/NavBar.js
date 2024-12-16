import React, { useState } from 'react';
import { ChevronRightIcon, ChevronLeftIcon, ChevronDownIcon, ChevronUpIcon } from '@heroicons/react/outline';

export default function NavBar() {
    const [isOpen, setIsOpen] = useState(true);
    const [isDropdownOpen, setIsDropdownOpen] = useState(false);

    const items = [
        { title: 'Users', link: '/users' },
        { title: 'Settings', link: '/settings' },
        { title: 'Reports', link: '/reports' }
    ];

    const toggleNav = () => {
        setIsOpen(!isOpen);
    };

    const toggleDropdown = () => {
        setIsDropdownOpen(!isDropdownOpen);
    };

    return (
        <div className="flex w-64">
            <nav
                className={`fixed top-0 left-0 h-full bg-gray-800 shadow-lg py-6 transition-all duration-300 ${isOpen ? 'w-64' : 'w-0'}`}
                style={{ overflow: 'hidden' }}
            >
                <a className="text-2xl font-bold text-white text-center block mb-8" href="/">
                    Admin Panel
                </a>

                <ul className="space-y-4">
                    <li>
                        <p
                            onClick={toggleDropdown}
                            className="block py-2 px-4 text-gray-300 hover:bg-gray-700 hover:text-white rounded-lg transition cursor-pointer"
                        >
                            Dashboard {isDropdownOpen ? (<ChevronUpIcon className="w-5 h-5 inline-block ml-1" />) : (<ChevronDownIcon className="w-5 h-5 inline-block ml-1" />)}
                        </p>
                        {isDropdownOpen && (
                            <ul className="space-y-2 pl-6 mt-2">
                                <li>
                                    <a
                                        className="block py-2 px-4 text-gray-300 hover:bg-gray-700 hover:text-white rounded-lg transition"
                                        href="/dashboard/overview"
                                    >
                                        Overview
                                    </a>
                                </li>
                                <li>
                                    <a
                                        className="block py-2 px-4 text-gray-300 hover:bg-gray-700 hover:text-white rounded-lg transition"
                                        href="/dashboard/stats"
                                    >
                                        Stats
                                    </a>
                                </li>
                            </ul>
                        )}
                    </li>

                    {items.map((item, index) => (
                        <li key={index}>
                            <a
                                className="block py-2 px-4 text-gray-300 hover:bg-gray-700 hover:text-white rounded-lg transition"
                                href={item.link}
                            >
                                {item.title}
                            </a>
                        </li>
                    ))}
                </ul>

                <div className="w-48 h-px bg-gray-600 my-4 mx-auto"></div>
            </nav>

            <button
                onClick={toggleNav}
                className="absolute top-1/2 transform -translate-y-1/2 w-8 h-16 text-white z-20 border-2 bg-gray-600 rounded-r-lg items-center justify-center flex"
                style={{
                    transition: 'transform 300ms',
                    left: isOpen ? '15rem' : '0',
                }}
            >
                {isOpen ? (
                    <ChevronLeftIcon className="w-8 h-10" />
                ) : (
                    <ChevronRightIcon className="w-8 h-10" />
                )}
            </button>
        </div>
    );
}

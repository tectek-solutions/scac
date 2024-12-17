import React, { useState } from 'react';
import { ChevronRightIcon, ChevronLeftIcon, ChevronDownIcon, ChevronUpIcon } from '@heroicons/react/outline';

export default function NavBar({ isOpen, setIsOpen }) {
    const [isDropdownOpen, setIsDropdownOpen] = useState(false);

    const items = [
        { title: 'Settings', link: '/settings' },
        { title: 'Reports', link: '/reports' }
    ];

    const tables = [
        { title: 'Users', link: '/tables/users' },
        { title: 'Authentications', link: '/tables/authentications' },
        { title: 'User Tokens', link: '/tables/user-tokens' },
        { title: 'Apis', link: '/tables/apis' },
        { title: 'Actions', link: '/tables/actions' },
        { title: 'Reactions', link: '/tables/reactions' },
        { title: 'Workflows', link: '/tables/workflows' },
        { title: 'Triggers', link: '/tables/triggers' }
    ];

    const [tablesOpen, setTablesOpen] = useState(false);

    const toggleNav = () => {
        setIsOpen(!isOpen);
    };

    const toggleDropdown = () => {
        setIsDropdownOpen(!isDropdownOpen);
    };


    const toggleTables = () => {
        setTablesOpen(!tablesOpen);
    }


    return (
        <div className={`flex ${isOpen ? 'w-64' : 'w-0'} h-full transition-all duration-300`}>
            <nav
                className={`fixed top-0 left-0 h-full bg-gray-800 shadow-lg py-6 transition-all duration-300 ${isOpen ? 'w-64' : 'w-0'}`}
                style={{ overflowY: 'auto', scrollbarWidth: 'none' }}
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

                <div className="w-56 h-px bg-gray-600 my-4 mx-auto"></div>

                <ul className="space-y-4">
                    <li>
                    <p
                            onClick={toggleTables}
                            className="block py-2 px-4 text-gray-300 hover:bg-gray-700 hover:text-white rounded-lg transition cursor-pointer"
                        >
                            Tables {tablesOpen ? (<ChevronUpIcon className="w-5 h-5 inline-block ml-1" />) : (<ChevronDownIcon className="w-5 h-5 inline-block ml-1" />)}
                        </p>
                        {tablesOpen && (
                            <ul className="space-y-2 pl-6 mt-2">
                                {tables.map((table, index) => (
                                    <li key={index}>
                                        <a
                                            className="block py-2 px-4 text-gray-300 hover:bg-gray-700 hover:text-white rounded-lg transition"
                                            href={table.link}
                                        >
                                            {table.title}
                                        </a>
                                    </li>
                                ))}
                            </ul>
                        )}
                    </li>
                </ul>
                
                
            </nav>

            <button
                onClick={toggleNav}
                className="absolute top-1/2 transform -translate-y-1/2 w-8 h-16 text-white border-2 bg-gray-600 rounded-r-lg items-center justify-center flex"
                style={{
                    transform: (isOpen ? 'translateX(15rem)' : 'translateX(0)'),
                    transition: 'transform 300ms'
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

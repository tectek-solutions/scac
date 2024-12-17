import React from "react";

export default function Input({ label, type, id, value, onChange, options }) {
    return (
        <div className="mb-6">
            <label htmlFor={id} className="block text-sm font-medium">
                {label}
            </label>
            {type === "select" ? (
                <select
                    id={id}
                    className="mt-1 block w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                    value={value}
                    onChange={onChange}
                >
                    {options.map((option) => (
                        <option key={option.value} value={option.value}>
                            {option.label}
                        </option>
                    ))}
                </select>
            ) : type === "text" ? (
                <textarea
                    id={id}
                    className="mt-1 block w-full h-10 p-2 border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                    value={value}
                    onChange={onChange}
                />
            ) : (
                <input
                    type={type}
                    id={id}
                    className="mt-1 block w-full p-2 border border-gray-300 rounded-md shadow-sm focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                    value={value}
                    onChange={onChange}
                />
            )}
        </div>
    );
}
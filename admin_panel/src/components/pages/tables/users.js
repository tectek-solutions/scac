import React, { useState } from "react";
import ActionsTab from "../../Functions/ActionsTab";
import FetchComponent from "../../Functions/Fetch";

export default function Users() {
    const [user, setUser] = useState([]);
    const [search, setSearch] = useState("");
    const [selected, setSelected] = useState([]);
    const [selectAll, setSelectAll] = useState(false);


    FetchComponent({ setValues: setUser, ApiUrl: "http://localhost:5000/api/users" });

    return (
        <div className="flex">
            <div className="flex-1 p-4">
                <ActionsTab ValueText="utilisateur" search={search} setSearch={setSearch} selected={selected} setSelected={setSelected} PathRedirect="/tables/users/add_user" UrlApi="http://localhost:5000/api/users" />
            </div>
        </div>
    )
};
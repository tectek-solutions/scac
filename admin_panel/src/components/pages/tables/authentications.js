import React, { useState } from "react";
import ActionsTab from "../../Functions/ActionsTab";
import FetchComponent from "../../Functions/Fetch";

export default function Authentications() {
    const [authentications, setAuthentications] = useState([]);
    const [search, setSearch] = useState("");
    const [selected, setSelected] = useState([]);
    const [selectAll, setSelectAll] = useState(false);


    FetchComponent({ setValues: setAuthentications, ApiUrl: "http://localhost:8000/authentications/" });

    console.log(authentications);

    return (
        <div className="flex">
            <div className="flex-1 p-4">
                <ActionsTab ValueText="authentication" search={search} setSearch={setSearch} selected={selected} setSelected={setSelected} PathRedirect="/tables/users/add_user" UrlApi="http://localhost:8000/authentications/" />
            </div>
        </div>
    )
};

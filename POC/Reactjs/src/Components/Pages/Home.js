import React, { useState } from "react";
import axios from 'axios';
import '../../App.css';

export default function Home() {
    const [data, setData] = useState([]);
    const [type, setType] = useState('');

    const get = () => {
        axios.get('https://jsonplaceholder.typicode.com/posts')
        .then((response) => {
            setData(response.data);
            setType('GET');
        })
        .catch((error) => {
            console.log(error);
        });
    }

    const post = () => {
        axios.post('https://jsonplaceholder.typicode.com/posts', {
            title: 'foo',
            body: 'bar',
            userId: 1
        })
        .then((response) => {
            setData([response.data]);
            setType('POST');
        })
        .catch((error) => {
            console.log(error);
        });
    }

    const put = () => {
        axios.put('https://jsonplaceholder.typicode.com/posts/1', {
            id: 1,
            title: 'Test put',
            body: 'lorem ipsum',
            userId: 1
        })
        .then((response) => {
            setData([response.data]);
            setType('PUT');
        })
        .catch((error) => {
            console.log(error);
        });
    }

    return (
        <div className="App">
            <div className="container">
                <div className="container-btn">
                    <button className="btn" onClick={get}>GET</button>
                    <button className="btn" onClick={post}>POST</button>
                    <button className="btn" onClick={put}>PUT</button>
                </div>
                <div className="result">
                    {type === 'GET' && (
                        <div className="get-results">
                            <h1>GET Results:</h1>
                            {data.map((item, index) => (
                                <div key={index} className="item">
                                    <h1>Title: {item.title}</h1>
                                    <p>Description: {item.body}</p>
                                </div>
                            ))}
                        </div>
                    )}
                    {type === 'POST' && (
                        <div className="post-results">
                            <h1>POST Result:</h1>
                            {data.map((item, index) => (
                                <div key={index} className="item">
                                    <h1>Title: {item.title}</h1>
                                    <p>Description: {item.body}</p>
                                </div>
                            ))}
                        </div>
                    )}
                    {type === 'PUT' && (
                        <div className="put-results">
                            <h1>PUT Result:</h1>
                            {data.map((item, index) => (
                                <div key={index} className="item">
                                    <h1>Title: {item.title}</h1>
                                    <p>Description: {item.body}</p>
                                </div>
                            ))}
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
};
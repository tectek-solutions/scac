import { useEffect } from 'react';
import axios from 'axios';

const FetchComponent = ({ setValues, ApiUrl }) => {
  useEffect(() => {
    const fetchData = async () => {
      try {
        const response = await axios.get(ApiUrl, {
          headers: {
            accept: "*/*",
            "Content-Type": "application/json",
          },
        });
        setValues(response.data);
      } catch (error) {
        console.error("Error fetching analyse:", error);
      }
    };

    fetchData();
  }, [ApiUrl, setValues]);

  return null;
};

export default FetchComponent;
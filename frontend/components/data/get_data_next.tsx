import { useState } from "react"
import { AuthFetch } from "../../services/auth_fetch"

export const GetDataNext = () => {
    let [dataResponse, setDataResponse] = useState<string>('');
    let [statusColour, setStatusColour] = useState<string>('grey');
    const getData = () => {
        AuthFetch.fetch('/api/data_js/get_data', {method: 'POST'})
        .then((response) => response.json())
        .then((data) => {
            console.log(data);
            // setDataResponse(data)
            setStatusColour('green')
        })
        .catch(error => {
            // setDataResponse(error)
            setStatusColour('red')
        })
        setTimeout(() => {
            setStatusColour('grey')
        }, 1000)
    }

    return <><div>
        <h2>Next data fetcher </h2>
        </div>
        <div>
        {dataResponse}
    </div>
    <button onClick={getData} style={{ backgroundColor: statusColour }}>Get Next Data</button></>
}

import { useState } from "react";
import { AuthFetch } from "../../../services/auth_fetch";

export const AccessTokenCheckAgainstAuth = () => {
    let [dataResponse, setDataResponse] = useState<string>('');
    let [statusColour, setStatusColour] = useState<string>('grey');
    const getData = () => {
        AuthFetch.fetch('/api/auth/verify_token', {method: 'POST'})
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
        <h2>Auth Server data fetcher </h2>
        </div>
        <div>
        {dataResponse}
    </div>
    <button onClick={getData} style={{ backgroundColor: statusColour }}>Check Token Against Auth Server</button></>
}
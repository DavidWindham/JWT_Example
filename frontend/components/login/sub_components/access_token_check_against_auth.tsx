import { useState } from "react";
import { AuthFetch } from "../../../services/auth_fetch";

export const AccessTokenCheckAgainstAuth = () => {
    let [dataResponse, setDataResponse] = useState<string>('');
    let [statusColour, setStatusColour] = useState<string>('grey');
    const getData = () => {
        AuthFetch.fetch('/api/auth/verify_token', {method: 'POST'})
            .then((response) => {
                if (response.status !== 202){
                    throw new Error('Invalid status code: ' + response.status);
                }
                return response.json()
            })
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

    return <>
        <div style={{marginBottom: "2rem"}}>
            Auth Server data fetcher
            <button onClick={getData} style={{ backgroundColor: statusColour, marginLeft: "1rem" }}>Check Token Against Auth Server</button>
        </div>
    </>
}
import { AuthFetch } from "../../../services/auth_fetch";
import { Dispatch, SetStateAction, useState } from "react"

export const AccessTokenCheckAgainstAuth = (props: genericProps) => {
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
                props.setLoggedIn(false)
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

interface genericProps {
    setLoggedIn: Dispatch<SetStateAction<boolean>>
}
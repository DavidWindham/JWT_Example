import { AuthFetch } from "../../services/auth_fetch"
import { Dispatch, SetStateAction, useState } from "react"

export const GetDataPython = (props: genericProps) => {
    let [statusColour, setStatusColour] = useState<string>('grey');
    const getData = () => {
        AuthFetch.fetch('/api/data_py/get_data', {method: 'POST'})
            .then((response) => {
                if (response.status !== 200){
                    throw new Error('Invalid status code: ' + response.status);
                }
                return response.json()
            })
            .then((data) => {
                console.log(data);
                setStatusColour('green')
            })
            .catch(error => {
                setStatusColour('red')
                props.setLoggedIn(false)
            })
            .finally(() => {
                setTimeout(() => {
                    setStatusColour('grey')
                }, 1000)
            })
    }

    return <>
        <div style={{marginBottom: "2rem"}}>
            Python data fetcher
            <button onClick={getData} style={{ backgroundColor: statusColour, marginLeft: "1rem"  }}>Get Python Data</button>
        </div>
    </>
}

interface genericProps {
    setLoggedIn: Dispatch<SetStateAction<boolean>>
}
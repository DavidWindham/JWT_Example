import { Love_Light } from "@next/font/google";
import { useState } from "react"
import { AuthFetch } from "../../services/auth_fetch"

export const GetDataPython = () => {
    let [dataResponse, setDataResponse] = useState<string>('');
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
            Python data fetcher
            <button onClick={getData} style={{ backgroundColor: statusColour, marginLeft: "1rem"  }}>Get Python Data</button>
        </div>
    </>
}
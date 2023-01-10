import { useState } from "react"
import { AuthFetch } from "../../services/auth_fetch"

export const GetDataNext = () => {
    let [dataResponse, setDataResponse] = useState<string>('');
    let [statusColour, setStatusColour] = useState<string>('grey');
    const getData = () => {
        AuthFetch.fetch('/api/data_js/get_data', {method: 'POST'})
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
            Next data fetcher
            <button onClick={getData} style={{ backgroundColor: statusColour, marginLeft: "1rem"  }}>Get Next Data</button>
        </div>
    </>
}

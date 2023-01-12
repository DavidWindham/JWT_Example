import { Dispatch, SetStateAction, useState } from "react";
import TokenStorage from "../../services/token_storage";

export const LoginRegisterParent = (props: login_register_props) => {
    let [username, setUsername] = useState<string>('')
    let [password, setPassword] = useState<string>('')

    let [loginStatusColour, setLoginStatusColour] = useState<string>('grey')
    let [registerStatusColour, setRegisterStatusColour] = useState<string>('grey')

    const onUsernameChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setUsername(event.target.value);
    }
    const onPasswordChange = (event: React.ChangeEvent<HTMLInputElement>) => {
        setPassword(event.target.value);
    }

    const login = async () => {
        const data = {
            username: username,
            password: password
        };

        const options = {
            method: 'POST',
            body: JSON.stringify(data),
            headers: {
                'Content-Type': 'application/json'
            }
        };

        fetch('/api/auth/login', options)
            .then(response => response.json())
            .then(data => {
                let bothTokenObject = {
                    "access_token": data.access_token,
                    "refresh_token": data.refresh_token
                }
                TokenStorage.storeAccessAndRefreshTokens(bothTokenObject)
                setLoginStatusColour('green')
                props.setLoggedIn(true);
            })
            .catch(error => {
                console.error(error)
                setLoginStatusColour('red')
            })
        
        setTimeout(() => {
            setLoginStatusColour('grey')
        }, 1000)
    }

    const register = async () => {
        const data = {
            username: username,
            password: password
        };

        const options = {
            method: 'POST',
            body: JSON.stringify(data),
            headers: {
                'Content-Type': 'application/json'
            }
        };

        fetch('/api/auth/register', options)
            .then(response => response.json())
            .then(data => {
                let bothTokenObject = {
                    "access_token": data.access_token,
                    "refresh_token": data.refresh_token
                }
                TokenStorage.storeAccessAndRefreshTokens(bothTokenObject)
                setRegisterStatusColour('green')
            })
            .catch(error => {
                console.error(error)
                setRegisterStatusColour('red')
            });

        setTimeout(() => {
            setRegisterStatusColour('grey')
        }, 1000)
    }

    return <>
        {props.loggedInStatus ? 
        <></> :
        <div>
            <div>
                <div>
                    Username: <input onChange={onUsernameChange}></input>
                </div>
                <div>
                    Password: <input onChange={onPasswordChange}></input> 
                </div>
            </div>
            <button onClick={login} style={{width:"50%", backgroundColor: loginStatusColour}}>Login</button>
            <button onClick={register} style={{width:"50%", backgroundColor: registerStatusColour}}>Register</button>
        </div>
    }
    </>
}

interface login_register_props{
    loggedInStatus: boolean
    setLoggedIn: Dispatch<SetStateAction<boolean>>
}
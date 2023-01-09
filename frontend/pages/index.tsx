import Head from 'next/head'
import Image from 'next/image'
import { Inter } from '@next/font/google'
import styles from '../styles/Home.module.css'
import { useEffect, useState } from 'react'

import { TokenStorage } from '../services/token_storage'
import { AuthFetch } from '../services/auth_fetch'

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  let [accessToken, setAccessToken] = useState<string|null>('')
  let [refreshToken, setRefreshToken] = useState<string|null>('')
  let [username, setUsername] = useState<string>('')
  let [password, setPassword] = useState<string>('')

  useEffect(() => {
    setAccessToken(TokenStorage.getAccessToken())
    setRefreshToken(TokenStorage.getRefreshToken())
  }, [])

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
      })
      .catch(error => console.error(error));
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

    fetch('/api/auth/refresh_token', options)
      .then(response => response.json())
      .then(data => console.log(data))
      .catch(error => console.error(error));
  }

  const checkAccessToken = async () => {
    let accessToken = TokenStorage.getAccessToken();
    if (accessToken != null) {
      const options = {
        method: 'POST',
        body: JSON.stringify({}),
        headers: {
          'Content-Type': 'application/json',
          'access_token': accessToken,
        }
      };

      AuthFetch.fetch('/api/auth/verify_token', options)
        .then(response => response.json())
        .then(data => console.log(data))
        .catch(error => console.error(error));
    }
  }

  const refreshAccessToken = async () => {
    const data = {
      refresh_token: TokenStorage.getRefreshToken()
    };

    const options = {
      method: 'POST',
      body: JSON.stringify(data),
      headers: {
        'Content-Type': 'application/json'
      }
    }

    AuthFetch.fetch('/api/auth/refresh_token', options)
      .then(response => response.json())
      .then(data => {
        TokenStorage.storeAccessToken(data.access_token)
      })
      .catch(error => console.error(error))
  }

  const getData = async () => {
    let accessToken = TokenStorage.getAccessToken();
    if (accessToken !== null){
      const data = {}

      const options = {
        method: 'POST',
        body: JSON.stringify(data),
        headers: {
          'Content-Type': 'application/json',
          'access_token': accessToken
        }
      }

      AuthFetch.fetch('/api/data/get_data', options)
        .then(response => response.json())
        .then(data => {
          console.log(data);
        })
        .catch(error => console.error(error))
    }
  }

  const onUsernameChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setUsername(event.target.value);
  }
  const onPasswordChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setPassword(event.target.value);
  }

  return (
    <>
      <Head>
        <title>JWT Example</title>
      </Head>
      <div>
        Username: <input onChange={onUsernameChange}></input>
        Password: <input onChange={onPasswordChange}></input> 
      </div>
      <div>
        Access Token: {accessToken}
      </div>
      <div>
      Refresh Token: {refreshToken}
      </div>

      <button onClick={login}>Login</button>
      <button onClick={register}>Register</button>
      <button onClick={checkAccessToken}>Check Access Token</button>
      <button onClick={refreshAccessToken}>Refresh Access Token</button>
      <button onClick={getData}>Get Data</button>
    </>
  )
}

import Head from 'next/head'
import Image from 'next/image'
import { Inter, Ribeye } from '@next/font/google'
import styles from '../styles/Home.module.css'
import { useEffect, useState } from 'react'

import { TokenStorage } from '../services/token_storage'
import { GetDataPython } from '../components/data/get_data_python'
import { GetDataNext } from '../components/data/get_data_next'
import { AccessTokenCheckAgainstAuth } from '../components/login/sub_components/access_token_check_against_auth'
import { LoginRegisterParent } from '../components/login/login_register_parent'

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  let [accessToken, setAccessToken] = useState<string|null>('')
  let [refreshToken, setRefreshToken] = useState<string|null>('')
  
  useEffect(() => {
    setAccessToken(TokenStorage.getAccessToken())
    setRefreshToken(TokenStorage.getRefreshToken())
  }, [])

  const setArtificialAccessToken = () => {
    TokenStorage.storeAccessToken('fake_token')
  }

  return (
    <>
      <Head>
        <title>JWT Example</title>
      </Head>
      <div style={{display: "flex", width: "100%", marginRight: "1rem"}}>
        <button onClick={setArtificialAccessToken}>SET ARTIFICIAL TOKEN</button>
        <LoginRegisterParent />
        <div>
          Data fetching area
          <GetDataPython />
          <GetDataNext />
          <AccessTokenCheckAgainstAuth />
        </div>
      </div>
    </>
  )
}

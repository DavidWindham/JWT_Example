import Head from 'next/head'
import Image from 'next/image'
import { Inter, Ribeye } from '@next/font/google'
import styles from '../styles/Home.module.css'
import { useEffect, useState } from 'react'

import { TokenStorage } from '../services/token_storage'
import { GetDataPython } from '../components/data/get_data_python'
import { GetDataNext } from '../components/data/get_data_next'
import { AccessTokenCheckAgainstAuth } from '../components/login_logout/sub_components/access_token_check_against_auth'
import { LoginRegisterParent } from '../components/login_logout/login_register_parent'

const inter = Inter({ subsets: ['latin'] })

export default function Home() {
  let [loggedIn, setLoggedIn] = useState<boolean>(false);

  const setArtificialAccessToken = () => {
    TokenStorage.storeAccessToken('fake_token')
  }

  return (
    <>
      <Head>
        <title>JWT Example</title>
      </Head>
      <div style={{width: "60%", marginLeft: "20%", marginRight: "20%", textAlign: "center"}}>
      <div>
          {/* <button onClick={setArtificialAccessToken}>SET ARTIFICIAL TOKEN</button> */}
          <LoginRegisterParent loggedInStatus={loggedIn} setLoggedIn={setLoggedIn}/>
      </div>
      <div style={{marginTop: "2rem", marginBottom: "1rem"}}>
        Data fetching area
      </div>
      <div style={{display: "flex", width: "100%", marginRight: "1rem", marginBottom: "5rem"}}>
        <GetDataPython setLoggedIn={setLoggedIn}/>
        <GetDataNext setLoggedIn={setLoggedIn}/>
        <AccessTokenCheckAgainstAuth setLoggedIn={setLoggedIn}/>
      </div>
      </div>
    </>
  )
}

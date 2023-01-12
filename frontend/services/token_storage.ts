import { AuthFetch } from "./auth_fetch"

export interface DualTokenObject {
    "access_token": string
    "refresh_token": string
}

const getFromStorage = (key: string) => {
    if(typeof window !== 'undefined'){
        return window.localStorage.getItem(key)
    }
    return ''
}

const setToStorage = (key: string, value: string) => {
    if(typeof window !== 'undefined'){
        return window.localStorage.setItem(key, value)
    }
}

const removeFromStorage = (key: string) => {
    if(typeof window !== 'undefined'){
        return window.localStorage.removeItem(key)
    }
}

export class TokenStorage {
  static ACCESS_TOKEN: string | null = getFromStorage('access_token')
  static REFRESH_TOKEN: string | null = getFromStorage('refresh_token')
  static shouldCheckRefresh: boolean = true

  static setRefreshFalse() {
    this.shouldCheckRefresh = false
  }

  static setRefreshTrue() {
    this.shouldCheckRefresh = true
  }

  static shouldTokenRefresh() {
    return this.shouldCheckRefresh
  }

  static isAuthenticated() {
    return this.getAccessToken() !== null
  }

  static getNewTokens() {
    this.unload_old_access_token()
    return new Promise((resolve, reject) => {
        const data = {
            refresh_token: this.getRefreshToken()
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
            .then(data => {
                this.storeAccessAndRefreshTokens({"access_token": data.access_token, "refresh_token": data.refresh_token})
                this.setRefreshFalse()
                resolve(data)
            })
            .catch(error => {
                console.error(error)
                this.setRefreshFalse()
                reject(error)
            })
    })
  }

  static storeAccessAndRefreshTokens(bothTokens:DualTokenObject) {
    this.clear()
    this.storeAccessToken(bothTokens.access_token)
    this.storeRefreshToken(bothTokens.refresh_token)
    this.setRefreshTrue()
  }

  static storeAccessToken(accessTokenString:string) {
    setToStorage('access_token', accessTokenString)
    this.ACCESS_TOKEN = accessTokenString
  }

  static storeRefreshToken(refreshTokenString:string) {
    setToStorage('refresh_token', refreshTokenString)
    this.REFRESH_TOKEN = refreshTokenString
  }

  static unload_old_access_token() {
    removeFromStorage('access_token')
    this.ACCESS_TOKEN = null
  }

  static clear() {
    removeFromStorage('access_token')
    removeFromStorage('refresh_token')
    this.ACCESS_TOKEN = null
    this.REFRESH_TOKEN = null
  }

  static getAccessToken() {
    return this.ACCESS_TOKEN
  }

  static getRefreshToken() {
    return this.REFRESH_TOKEN
  }
}

export default TokenStorage
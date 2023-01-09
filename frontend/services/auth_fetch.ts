import { TokenStorage } from "./token_storage"

interface FetchOptions extends RequestInit {
  headers?: Headers | { [key: string]: string }
}

export const AuthFetch = {
  fetch(url: string, options?: FetchOptions): Promise<Response> {
    return fetch(url, options).then(response => {
      if (response.status === 401) {
        // request returned a 401 status, handle this case
        console.log("Refresh status: ", TokenStorage.shouldTokenRefresh())
        if (TokenStorage.shouldTokenRefresh()){
          TokenStorage.setRefreshFalse()
          return TokenStorage.getNewToken()
            .then(() => {
              let newToken = TokenStorage.getAccessToken()
              if (newToken != null) {
                TokenStorage.setRefreshTrue()
                const newOptions = {
                  ...options,
                  headers: {
                    ...options?.headers,
                    access_token: newToken
                  }
                };
                return AuthFetch.fetch(url, newOptions)
              }
              return Promise.reject()
            })
            .catch((err) => {
              // there was an error getting the new token, reject the Promise
              console.log("Failed to get new access token")
              return Promise.reject(err)
            });
        } else {
          return response
        }
      } else {
        // request was successful, return the response
        return response
      }
    });
  }
};

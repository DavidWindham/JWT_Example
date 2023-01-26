
## Environment Variables

We have 6 environment variables for this service.

`DATABASE_URL` e.g. `DATABASE_URL=postgres://<username>:<password>@<url>/<table_name>`
  The address of the Postgres Database. It also needs the username, password, and database name

`PASSWORD_SALT`
  This is the salt used to generate password hashes for users. The default is password-salt

`ACCESS_TOKEN_EXPIRE_SECONDS`
  How long until an access token expires after creation

REFRESH_TOKEN_EXPIRE_SECONDS
  How long until a refresh token expires after creation
  
ACCESS_TOKEN_SECRET
  By default this is set to 'access-token-secret'. If this is changed, it must be changed for all services being used
  
REFRESH_TOKEN_SECRET
  This is only used for refresh token creation/validation, so this is unique to this service (or the Lambda service)
  
  
## Deployment

This section will be updated at a later date, as the deployment is complicated due to postgres

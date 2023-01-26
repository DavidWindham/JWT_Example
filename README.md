# JWT_Example

We have 4 individual services within this project and all use environment variables for the access token secret. Each readme will have a brief section about how to set the environment variable.

### Rust based authentication service
* Generates JWT's
* Leverages a Postgres database to store user records


### Typescript Node Lambda based authentication service
* Generates JWT's
* Leverages DynamoDB to store user records


### NextJS Application
##### Frontend
* AuthFetch - fetch wrapper to automatically refresh access/refresh JWTs
* TokenStorage to pair with the fetch wrapper to store and handle tokens
##### Backend
* Utilizes httpProxyMiddleware with API routes to tie all systems together
* Also has an endpoint that validates access token


### Python Flask microservice
* validates a token and returns data, mocking a DATA API
A Rust based or Node with Lambda based JWT authentication system, a Next hosted Frontend, and a third Python based API to check if token is valid elsewhere


Navigate into any of the project folders for a detailed readme on the functionality of each area

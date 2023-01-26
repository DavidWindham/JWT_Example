
## Environment Variables

We have 3 environment variables for this service within the .env file.

`AUTH_API_ADDRESS`
  The address of either the Rust auth API, or the Lambda API Gateway. Do n ote the gateway must have "/api/" on the end
  
`AWS_GATEWAY_API_KEY`
  This is the key seen during deployment of the lambda authentication system. This isn't  required if you're using the Rust service
  
`DATA_API_ADDRESS`
  The address of the Python data API endpoint
  
 `ACCESS_TOKEN_SECRET`
  By default this is set to 'access-token-secret'. If this is changed, it must be changed for all services being used



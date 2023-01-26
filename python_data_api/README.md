
## Environment Variables

We only have 1 environment variable for this service within the .env file

`ACCESS_TOKEN_SECRET`
  By default this is set to 'access-token-secret'. If this is changed, it must be changed for all services being used
  
 

If you wish to see what happens if the signatures don't match, then change this value to anything else, in the .env file you can see an example of this


## Deployment

Just run these 2 commands: 
1. `pip install -r requirements.txt` to install the dependencies
2. `python main.py` to run the microservice

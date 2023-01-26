# Lambda Authentication written in Node

We leverage the following technologies to to simulate a very basic workflow for JWT Authentication:
* Cloudformation Template
* API Gateway
* API Key Authorization and Usage Plan defined in Template
* Lambda Functions
* DynamoDB
* Lambda Layers
* Environment Variables defined in template

This example has some serious drawbacks, do not use this, this is for demo purposes only
* All passwords are stored in plain text currently
* Performance is absolutely terrible for this use case
* Token creation is messy at best
* Code redundancy
* Poor project structure

But with that disclaimer out the way, let's get on with deploying this.

## Project Structure Description

The functions are the 4 .ts files in the `/src` folder. They each have the function "handler" that's referenced in the template.yml with the line "Handler: register.handler". This is what defines the entrypoint to each lambda function.

All 4 functions have the dependency jwt.ts within the dependencies folder. Because this is a local dependency, when the project is compiled, this code will be included in all compiled functions. This is different to the external dependencies for which we use a lambda layer.

The functions will be compiled during deployment, but if you want to compile it yourself, run `npm run build` in /src. If you want it to recompile instantly as you make changes to the .ts files, navigate to /src and run `npm run watch`. This will watch for changes and automatically build continuously.

The lambda layer `all-dependencies` is located with `/src/layers/` and will be built with the second shell script (seen in deployment)

Everthing else in the project is described in the template.yaml or is generated from the build scripts (the bucket ID and the lambda layer ARN).

## Environment Variables

There are 2 Environment Variables that are set in the `template.yaml`, the ACCESS_TOKEN_SECRET and REFRESH_TOKEN_SECRET. These are at the top of the .yml file for ease of access. By default it's set to "access-token-secret", ensure if you change this, that you change it for all other services within this system.

## Deployment

This assumes you have AWS-CLI setup and you're logged in with an IAM user with the correct permissions.

To start, we'll build our solutions. Navigate to 
First off, you'll need permission to execute the shell scripts. Start by chmod +x all 3 deployment scripts
```shell
chmod +x 1_initial_setup.sh
chmod +x 2_build_and_deploy_lambda_layer.sh
chmod +x 3_deploy.sh
```
These scripts will handle the hard work of installing and deploying almost every part of the application

Let's start by running `./1_initial setup.sh`. This will generate a text file `bucket-name.txt` in this directory. This is where your functions will be deployed to later, so don't delete this file.

Next, we'll run `./2_build_and_deploy_lambda_layer.sh`. This will generate a text file `layer-info.txt` in this directory. Within this file, find the LayerVersionArn in the format `arn:aws:lambda:${AWS::Region}:${AWS::AccountId}:layer:all-dependencies-layer:<VERSION>`. The important part is the version on the end. There's no automatic way to do this next part within this project, so you'll need to ammend the template.yml file manually.

Within the template.yml file there's 4 references to this dependency, one for each AWS::Serverless::Function. You'll need to update the version number of each with the version number you saw in layer-info.txt

Once you've set those and saved the file, we're ready to deploy. Run `./3_deploy.sh`. Once this has finished, we should be ready to go.

Assuming we had no errors, we're finished desktop side, and we're ready to make our final changes AWS side.

First, let's associate our usage plan with the API and get our API Key
```
1. Login to AWS and navigate to API Gateway
2. Find dwin-node-typescript-authentication-api in your list of API's and open it
3. On the left, select "Usage Plans"
4. Within here, we need to associate this usage plan with the current API stage
5. Select "Add API Stage" and select our API and the stage
6. Ensure you click the tick on the right to save changes
7. Now our usage plan is set, navigate to "API Keys" on the far left and select our TestAuthKey
8. In the middle find the field "API key" with a "show" button next to it
9. Click the show button and take a note of this key
```
Now we have our usage plan registered and our API key, we just need the Invoke URL
```
1. Navigate to Stages on the left and select our stage, "api"
2. At the top you should see "Invoke URL:  https://<HASH>.execute-api.eu-west-2.amazonaws.com/api"
3. Take a note of this URL and let's give this a test
```

## Testing:

Replace the URL and the <API_KEY> with those aquired during deployment
```
curl -X POST 
        https://<HASH>.execute-api.eu-west-2.amazonaws.com/api/register 
        --header 'Content-Type: application/json' 
        --header 'x-api-key:<API_KEY>' 
        --data '{"username": "example_username", "password": "example_password"}'
```
If everything's worked, then using this CURL should return an access token and a refresh token.

## Endpoints

Here's a brief overview of each endpoint and their required arguments
For authorization to the API, all endpoints will need this header: { "x-api-key": "<API_KEY>"}

#### /register
The Username must be unique, you cannot register 2 of the same username
```
Headers: Content-Type: application/json
Body: { "username": "test_username", "password": "test_password" }
```
```
Success: Code 202 - Will return access_token and refresh_token
Failure: Code 400 - User already found
```

#### /login
The user must be registered and the password must match that found in the database
```
Headers: Content-Type: application/json
Body: { "username": "test_username", "password": "test_password" }
```
```
Success: Code 202 - Will return access_token and refresh_token
Failure: Code 400 - username not found, or password was incorrect
```

#### /verify_token
Used to check if the access_token provided from login/registration is currently valid
```
Headers: access_token: <ACCESS_TOKEN_FROM_LOGIN/REGISTER>
Body: None
```
```
Success: Code 202 - Token was valid
Failure: Code 401 - Token was invalid (expired/wrong sig/malformed)
```

#### /refresh_token
Used to generate new access and refresh tokens. The refresh tokens stay alive for 30 seconds and this route will return a new refresh token, so as long as you refresh within the last 30 seconds, you can extend the login indefinitely
```
Headers: Content-Type: application/json
Body: { "refresh_token": "<REFRESH_TOKEN_FROM_LOGIN/REGISTER" }
```
```
Success: Code 202 - Will also return new access_token and refresh_token
Failure: Code 400 - Refresh Token was expired/invalid
```

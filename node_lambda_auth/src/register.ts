import { DynamoDB } from "aws-sdk";
const dynamoDb = new DynamoDB.DocumentClient()
import { Handler } from "aws-lambda";
import { generateAccessToken, generateRefreshToken } from "./dependencies/jwt";
import { v4 as uuidv4 } from 'uuid';


export const handler: Handler = async (event: any, context: any) => {
    try {
        let body = JSON.parse(event.body)
        let id = uuidv4();
        let username = body['username']
        let password = body['password']

        let isUserAlreadyRegisteredData = await getUserInDBQuery(username)

        if (isUserAlreadyRegisteredData === undefined) {
            return{
                statusCode: 500,
                body: JSON.stringify({"Error": "Could not query DB"})
            }
        }

        // A length of 1 or more indicates the user is already present in the DB
        if (isUserAlreadyRegisteredData.Items?.length != 0){
            return {
                statusCode: 400,
                body: JSON.stringify({"Error": "User already found"})
            }; 
        }

        const params = {
            TableName: 'UserTable',
            Item: {
                id: id,
                username: username,
                password_hash: password
            }
        }

        let _newUserPutResponse = await dynamoDb.put(params).promise();

        const access_token_secret = process.env.ACCESS_TOKEN_SECRET
        const refresh_token_secret = process.env.REFRESH_TOKEN_SECRET

        if (access_token_secret === undefined || refresh_token_secret === undefined){
            return {
                statusCode: 500,
                body: JSON.stringify({"error": "Server error, access/refresh token keys not set properly"})
            }
        }

        const accessToken = generateAccessToken(access_token_secret, username)
        const refreshToken = generateRefreshToken(refresh_token_secret, username)

        return {
            statusCode: 202,
            body: JSON.stringify({"status": "ok", "access_token": accessToken, "refresh_token": refreshToken})
        }
    } catch (error) {
        return {
            statusCode: 500,
            body: JSON.stringify({"error": error, "message":"Caught error, possibly with inserting new user"})
        }
    }
}

const getUserInDBQuery = async (username: string) => {
    const check_params = {
        TableName: 'UserTable',
        ExpressionAttributeValues: {
            ":a": username
        },
        FilterExpression: "username = :a",
    };

    return await dynamoDb.scan(check_params).promise();
}
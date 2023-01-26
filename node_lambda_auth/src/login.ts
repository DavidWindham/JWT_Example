import { generateAccessToken, generateRefreshToken } from './dependencies/jwt.js'
import { Handler } from "aws-lambda";
import { DynamoDB } from "aws-sdk";
const dynamoDb = new DynamoDB.DocumentClient()


export const handler: Handler = async (event: any) => {
    try {
        let body = JSON.parse(event.body)

        // ok, a query must have both keys in, in this case "id" and "username"
        // It seems that "scan" perhaps is the term to use if you only have 1 of these, talking about hash or range, not actual password
        const params = {
            TableName: 'UserTable',
            ExpressionAttributeValues: {
                ":a": body['username']
            },
            FilterExpression: "username = :a",
        };

        let data = await dynamoDb.scan(params).promise();

        if (data === undefined) {
            return{
                statusCode: 500,
                body: JSON.stringify({"Error": "Could not query DB"})
            }
        }

        if (data.Items?.length != 1){
            return {
            statusCode: 400,
                body: JSON.stringify({"Error": "Username not found in db"})
            }; 
        }

        let user = data.Items[0]
        if (user.password_hash != body['password']) {
            return {
                statusCode: 400,
                body: JSON.stringify({"Error": "Invalid Password"})
            }
        }

        const access_token_secret = process.env.ACCESS_TOKEN_SECRET
        const refresh_token_secret = process.env.REFRESH_TOKEN_SECRET

        if (access_token_secret === undefined || refresh_token_secret === undefined){
            return {
                statusCode: 500,
                body: JSON.stringify({"error": "Server error, access/refresh token keys not set properly"})
            }
        }

        const accessToken = generateAccessToken(access_token_secret, body['username'])
        const refreshToken = generateRefreshToken(refresh_token_secret, body['username'])

        return {
            statusCode: 202,
            body: JSON.stringify({"Status": "ok", "access_token": accessToken, "refresh_token": refreshToken})
        }

    } catch (error) {
        return {
            statusCode: 500,
            body: JSON.stringify(error)
        };
    }
};

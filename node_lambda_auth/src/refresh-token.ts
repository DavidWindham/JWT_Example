import { Handler } from "aws-lambda";
import { generateAccessToken, generateRefreshToken, validateToken } from "./dependencies/jwt";


export const handler: Handler = async (event: any, context: any) => {
    console.log("This is a test")
    try {
        const body = JSON.parse(event.body)
        const encodedRefreshToken = body['refresh_token']

        let verification = validateToken("refresh-token-secret", encodedRefreshToken)

        if (verification.code === -1) {
            return {
                statusCode: 400,
                body: JSON.stringify({"Verification Status": verification})
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

        let decodedToken: any = verification.decoded_token
        const username = decodedToken.username

        const accessToken = generateAccessToken(access_token_secret, username)
        const refreshToken = generateRefreshToken(refresh_token_secret, username)

        return {
            statusCode: 200,
            body: JSON.stringify({"Verification Status": verification, "access_token": accessToken, "refresh_token": refreshToken, "decodedUsername": username})
        }
    } catch (error) {
        return {
            statusCode: 500,
            body: JSON.stringify({"error": error, "other":"other"})
        }
    }
}
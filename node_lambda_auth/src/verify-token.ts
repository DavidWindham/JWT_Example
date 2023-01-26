import { Handler } from "aws-lambda";
import { generateAccessToken, generateRefreshToken, validateToken } from "./dependencies/jwt";


export const handler: Handler = async (event: any, context: any) => {
    console.log("This is a test")
    try {
        const encodedToken = event.headers.access_token

        const access_token_secret = process.env.ACCESS_TOKEN_SECRET

        if (access_token_secret === undefined){
            return {
                statusCode: 500,
                body: JSON.stringify({"error": "Server error, access token keys not set properly"})
            }
        }

        let verification = validateToken(access_token_secret, encodedToken)

        if (verification.code === -1){
            return {
                statusCode: 401,
                body: JSON.stringify({"Verification Status": verification})
            }
        }

        return {
            statusCode: 202,
            body: JSON.stringify({"Verification Status": verification})
        }
    } catch (error) {
        return {
            statusCode: 500,
            body: JSON.stringify({"error": error})
        }
    }
}
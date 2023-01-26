import { JwtPayload, sign } from 'jsonwebtoken'
import { JsonWebTokenError, TokenExpiredError, verify as jwtVerify } from 'jsonwebtoken'


export const generateAccessToken = (secret: string, username: string) => {
    return generateToken(secret, username, 5)
}

export const generateRefreshToken = (secret: string, username: string) => {
    return generateToken(secret, username, 30)
}

const generateToken = (secret: string, arg: string, lifespan_seconds: number) => {
    const payload = { 
        exp: Math.floor(Date.now() / 1000) + (lifespan_seconds),
        iss: "lambda authenticator",
        department: "EXAMPLE_DEPARTMENT",
        username: arg 
    };
    const token = sign(payload, secret, { algorithm: 'HS384' });
    return token;
}

export const validateToken = (access_token_secret: string, encryptedToken: string) => {
    console.log("Encrypted token: ", encryptedToken);
    if (encryptedToken === null || encryptedToken === undefined){
        return {
            "code": -1,
            status: "Token was not found",
            decoded_token: {}
        }
    }

    if (Array.isArray(encryptedToken)){
        return {
            "code": -1,
            status: "Token was malformed",
            decoded_token: {}
        }
    }

    let decodedToken: string|JwtPayload = {};
    try {
        if (access_token_secret === null || access_token_secret === undefined){
            return {
                "code": -1,
                status: "Server error, cannot decode token",
                decoded_token: {}
            }
        }
        decodedToken = jwtVerify(encryptedToken, access_token_secret)
    }
    catch(err) {
        if (err instanceof TokenExpiredError){
            return {
                "code": -1,
                status: "Token was expired",
                decoded_token: {}
            }
        }
        if (err instanceof JsonWebTokenError) {
            return {
                "code": -1,
                status: "Token was invalid, invalid signiature",
                decoded_token: {}
            }
        }   
        return {
            "code": -1,
            status: "Unwritten Exception in Token Decode",
            decoded_token: {}
        }
    }
    return {
        "code": 0,
        status: "Token validation success",
        decoded_token: decodedToken
    }
}
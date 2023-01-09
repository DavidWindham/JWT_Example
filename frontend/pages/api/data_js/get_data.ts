import type { NextApiRequest, NextApiResponse } from 'next'
import {verify as jwtVerify } from 'jsonwebtoken'

type Data = {
  status: string
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  console.log("This is within the get data call")
  let encryptedToken = req.headers['access_token']
  console.log("Encrypted token: ", encryptedToken);
  if (encryptedToken === null || encryptedToken === undefined){
    return res.status(400).json({ status: 'Token was not found' })
  }

  if (Array.isArray(encryptedToken)){
    return res.status(400).json({ status: 'Headers were malformed' })
  }

  try {
    let access_token_secret = process.env.ACCESS_TOKEN_SECRET
    if (access_token_secret === null || access_token_secret === undefined){
      return res.status(500).json({ status: 'Server error, cannot decode token' })
    }
    jwtVerify(encryptedToken, access_token_secret)
  }
  catch {
    console.log("Invalid token")
    return res.status(401).json({ status: 'Token was expired or is invalid' })
  }

  return res.status(200).json({ status: 'Token validation success within Next API' })
}

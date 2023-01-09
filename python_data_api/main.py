from flask import Flask, request, jsonify
import jwt

import os
from os.path import join, dirname
from dotenv import load_dotenv

dotenv_path = join(dirname(__file__), '.env')
load_dotenv(dotenv_path)
ACCESS_TOKEN_SECRET = os.environ.get("ACCESS_TOKEN_SECRET")

app = Flask(__name__)


def check_access_token(func):
    def wrapper(*args, **kwargs):
        # Is token in header set
        if "access_token" not in request.headers:
            print("Access token was not in headers")
            return jsonify({"error": "Token is required in the header"}), 401
        
        # Is token valid? (expired, ect)
        encoded_jwt = request.headers["access_token"]
        try:
            jwt.decode(encoded_jwt, ACCESS_TOKEN_SECRET, algorithms=["HS384"])
        except jwt.exceptions.ExpiredSignatureError:
            return jsonify({"error": "Token was not valid"}), 401
        except jwt.exceptions.DecodeError:
            return jsonify({"error": "Failed to decode token, tampered with?"}), 401
        except : 
            return jsonify({"error": "I have no idea what you've done to get this error"}), 500

        return func(*args, **kwargs)
    return wrapper


@app.route("/get_data", methods=["POST"])
@check_access_token
def get_data():
    return jsonify({"data": "Some test data showing that you were authorized to access this information"})


if __name__ == "__main__":
    app.run(host="0.0.0.0", port=5050)

"""Application configuration for the API version diff scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "api-version-secret"

# user_id -> {username, email, password_hash, api_token}
USERS = {
    "u1": {
        "username": "alice",
        "email": "alice@example.com",
        "password_hash": "$2b$12$fakehashalicefakehashalice",
        "api_token": "tok-alice-abc123",
    },
    "u2": {
        "username": "bob",
        "email": "bob@example.com",
        "password_hash": "$2b$12$fakehashbobfakehashbob1234",
        "api_token": "tok-bob-def456",
    },
}

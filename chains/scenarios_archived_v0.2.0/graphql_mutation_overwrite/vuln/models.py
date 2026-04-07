"""Data models for the GraphQL-style mutation service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "gql-secret"

# user_id -> {username, email, role, subscription}
USERS = {
    "u1": {"username": "alice", "email": "alice@example.com", "role": "user", "subscription": "free"},
    "u2": {"username": "bob",   "email": "bob@example.com",   "role": "user", "subscription": "pro"},
}

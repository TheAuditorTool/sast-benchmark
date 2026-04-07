"""Application configuration for the GraphQL introspection scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "gql-introspect-secret"

# user_id -> {username, role, reset_token}
USERS = {
    "u1": {"username": "alice", "role": "user",  "reset_token": None},
    "u2": {"username": "bob",   "role": "user",  "reset_token": None},
    "a1": {"username": "root",  "role": "admin", "reset_token": "reset-root-abc123"},
}

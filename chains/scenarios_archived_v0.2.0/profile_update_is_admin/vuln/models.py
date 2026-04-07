"""Data models for the user profile service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "profile-secret"

# user_id -> {username, email, bio, is_admin}
USERS = {
    "1": {"username": "alice", "email": "alice@example.com", "bio": "Hi", "is_admin": False},
    "2": {"username": "bob",   "email": "bob@example.com",   "bio": "Hey", "is_admin": False},
    "99": {"username": "root", "email": "root@example.com",  "bio": "",    "is_admin": True},
}

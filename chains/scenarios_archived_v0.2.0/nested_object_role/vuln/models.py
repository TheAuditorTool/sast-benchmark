"""Data models for the nested profile settings service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "nested-secret"

# user_id -> {username, settings: {theme, role}}
USERS = {
    "u1": {"username": "alice", "settings": {"theme": "dark",  "role": "user"}},
    "u2": {"username": "bob",   "settings": {"theme": "light", "role": "user"}},
}

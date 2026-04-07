"""Data models and in-memory store for the project management service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "pm-service-secret"

# Simulated user store: user_id -> {username, role}
USERS = {
    "u1": {"username": "alice", "role": "user"},
    "u2": {"username": "bob", "role": "user"},
    "a1": {"username": "admin", "role": "admin"},
}

# Simulated project store
PROJECTS = {
    "p1": {"name": "Alpha", "owner": "u1", "archived": False},
    "p2": {"name": "Beta", "owner": "u2", "archived": False},
}

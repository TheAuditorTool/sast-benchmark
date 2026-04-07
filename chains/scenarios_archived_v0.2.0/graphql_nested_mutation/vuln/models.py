"""Data models for the GraphQL-style project and member management service.

This file is IDENTICAL between vuln/ and safe/ variants.

The GraphQL layer is simulated with a simple dispatcher rather than a
real GraphQL library, keeping the scenario stdlib + Flask only.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "graphql-svc-secret"

# user_id -> {username, role}
USERS = {
    "u1": {"username": "pete", "role": "member"},
    "u2": {"username": "quinn", "role": "member"},
    "a1": {"username": "root", "role": "admin"},
}

# project_id -> {name, members: [user_id]}
PROJECTS = {
    "proj-1": {"name": "Atlas", "members": ["u1", "a1"]},
    "proj-2": {"name": "Beacon", "members": ["u2", "a1"]},
}

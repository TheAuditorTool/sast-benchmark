"""Application configuration for the server header version scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "server-header-secret"

SERVER_VERSION = "Flask/2.0.1 Python/3.9.0"

# username -> {password, role}
USERS = {
    "alice": {"password": "alicepass", "role": "user"},
    "admin": {"password": "adminpass", "role": "admin"},
}

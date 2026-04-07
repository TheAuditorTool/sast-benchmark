"""Application configuration for the username enumeration scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "enum-secret"

# username -> {password, role, email}
USERS = {
    "alice": {"password": "alicepass", "role": "user",  "email": "alice@example.com"},
    "admin": {"password": "adminpass", "role": "admin", "email": "admin@example.com"},
}

"""Application configuration for the Swagger endpoint leak scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "swagger-secret"

ADMIN_TOKEN = "super-admin-token-xyz"

# username -> {password, role}
USERS = {
    "alice": {"password": "alicepass", "role": "user"},
    "admin": {"password": "adminpass", "role": "admin"},
}

"""Application configuration for the reset token in response scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask
import secrets

app = Flask(__name__)
app.secret_key = "reset-secret"

# username -> {email, password, reset_token}
USERS = {
    "alice": {"email": "alice@example.com", "password": "oldpass", "reset_token": None},
    "bob":   {"email": "bob@example.com",   "password": "bobpass",  "reset_token": None},
}

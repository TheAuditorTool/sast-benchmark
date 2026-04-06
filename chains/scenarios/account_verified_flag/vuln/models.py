"""Data models for the account verification service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "verify-secret"

# user_id -> {email, email_verified, display_name}
USERS = {
    "u1": {"email": "alice@example.com", "email_verified": False, "display_name": "Alice"},
    "u2": {"email": "bob@example.com",   "email_verified": True,  "display_name": "Bob"},
}

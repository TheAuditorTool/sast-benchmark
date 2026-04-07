"""Application configuration for the CORS credential reflection scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "cors-secret"

# Simulated session store: session_id -> {user_id, role, api_token}
SESSIONS = {
    "sess-alice-111": {"user_id": "u1", "role": "user",  "api_token": "tok-alice-secret"},
    "sess-admin-999": {"user_id": "a1", "role": "admin", "api_token": "tok-admin-secret"},
}

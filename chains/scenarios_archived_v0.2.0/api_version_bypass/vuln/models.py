"""Data models for the notification and user management service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "notif-svc-secret"

# user_id -> {username, email, role, notifications_enabled}
USERS = {
    "u1": {"username": "judy", "email": "judy@corp.com", "role": "user", "notifications_enabled": True},
    "u2": {"username": "kate", "email": "kate@corp.com", "role": "user", "notifications_enabled": False},
    "a1": {"username": "admin", "email": "admin@corp.com", "role": "admin", "notifications_enabled": True},
}

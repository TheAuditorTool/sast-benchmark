"""Data models for the feature flag and settings management service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "settings-svc-secret"

# user_id -> {username, role}
USERS = {
    "u1": {"username": "liam", "role": "user"},
    "u2": {"username": "mia", "role": "user"},
    "a1": {"username": "ops", "role": "admin"},
}

# Feature flags: flag_name -> {enabled, admin_only}
# admin_only=True means only admins may toggle this flag
FEATURE_FLAGS = {
    "dark_mode": {"enabled": True, "admin_only": False},
    "beta_export": {"enabled": False, "admin_only": False},
    "maintenance_mode": {"enabled": False, "admin_only": True},
    "disable_signups": {"enabled": False, "admin_only": True},
}

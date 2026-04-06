"""Data models for the user preferences service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "prefs-secret"

# user_id -> {theme, notifications_enabled, feature_flags}
PREFERENCES = {
    "u1": {"theme": "dark", "notifications_enabled": True, "feature_flags": []},
    "u2": {"theme": "light", "notifications_enabled": False, "feature_flags": []},
}

ADMIN_FLAGS = {"beta_admin_panel", "unlimited_exports", "internal_metrics"}

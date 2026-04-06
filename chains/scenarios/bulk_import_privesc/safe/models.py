"""Data models for the bulk user import service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "bulk-secret"

# username -> {email, role}
USERS = {}

VALID_ROLES = {"user", "moderator", "admin"}
DEFAULT_ROLE = "user"

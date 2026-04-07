"""Data models for the user registration service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "reg-secret"

# username -> {password, role, email}
USERS = {}

VALID_ROLES = {"user", "moderator", "admin"}

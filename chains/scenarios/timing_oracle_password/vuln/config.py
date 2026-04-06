"""Application configuration for the timing oracle scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "timing-secret"

# username -> {password, role}
USERS = {
    "alice": {"password": "correct-horse-battery-staple", "role": "user"},
    "admin": {"password": "sup3r-s3cr3t-p@ssw0rd",        "role": "admin"},
}

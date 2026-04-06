"""Data models for the invite-only team collaboration service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "collab-secret-key"

# Valid invite tokens (single-use in a real system)
INVITE_TOKENS = {"INV-ALPHA-001", "INV-BETA-002"}

# Registered users: username -> {email, role, invite_used}
USERS = {}

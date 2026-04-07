"""Data models for the organisation membership service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "org-secret"

# org_id -> {name, owner_id}
ORGS = {
    "org1": {"name": "Acme", "owner_id": "u1"},
}

# membership_id -> {org_id, user_id, role}
MEMBERSHIPS = {}
_next_id = 1

"""Data models for the HR profile management service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "hr-service-secret"

# user_id -> profile dict
PROFILES = {
    "emp-001": {"name": "Grace", "email": "grace@corp.com", "role": "employee", "department": "Engineering"},
    "emp-002": {"name": "Heidi", "email": "heidi@corp.com", "role": "employee", "department": "HR"},
    "mgr-001": {"name": "Ivan", "email": "ivan@corp.com", "role": "manager", "department": "Engineering"},
}

# Allowlist of fields an employee may edit on their own profile
SELF_EDITABLE_FIELDS = {"name", "email", "department"}

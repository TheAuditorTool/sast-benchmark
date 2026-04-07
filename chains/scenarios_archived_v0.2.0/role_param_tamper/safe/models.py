"""Data models for the SaaS billing and team management service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "billing-svc-secret"

VALID_ROLES = {"member", "billing_admin", "owner"}

# username -> {email, role, org}
MEMBERS = {}

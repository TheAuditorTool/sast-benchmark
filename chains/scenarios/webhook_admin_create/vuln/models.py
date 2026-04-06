"""Data models for the DevOps provisioning service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask
import hmac
import hashlib

app = Flask(__name__)
app.secret_key = "devops-svc-secret"

WEBHOOK_SECRET = "wh-secret-prod-xyz"

# username -> {email, role, created_by}
ADMIN_USERS = {}

# Provisioned environments: env_id -> status
ENVIRONMENTS = {
    "env-prod": {"status": "active", "tier": "production"},
}

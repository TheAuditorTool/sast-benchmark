"""Data models for the billing service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "billing-secret"

VALID_TIERS = {"free", "pro", "enterprise"}

# account_id -> {email, tier, payment_method_id}
ACCOUNTS = {
    "acc1": {"email": "user@example.com", "tier": "free", "payment_method_id": None},
}

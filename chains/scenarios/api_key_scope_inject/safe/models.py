"""Data models for the API key management service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "apikey-secret"

# key_id -> {owner_id, name, scopes, active}
API_KEYS = {}
_counter = [1]

VALID_SCOPES = {"read", "write", "admin", "billing"}

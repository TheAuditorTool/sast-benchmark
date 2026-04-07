"""Flask application factory for the account-deletion CSRF scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "dev-secret-key"

import routes  # noqa: E402, F401 -- registers routes on app

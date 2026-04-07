"""Flask application factory for the SameSite-bypass CSRF scenario.

SameSite=Lax cookies are sent on top-level GET navigations, so a GET-based
state-change endpoint is still vulnerable to CSRF even with Lax cookies.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "dev-secret-key"
app.config["SESSION_COOKIE_SAMESITE"] = "Lax"

import routes  # noqa: E402, F401 -- registers routes on app

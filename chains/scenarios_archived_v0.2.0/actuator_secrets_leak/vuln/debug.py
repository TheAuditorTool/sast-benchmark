"""Actuator /env endpoint -- VULNERABLE variant.

Serves the full process environment including database passwords, cache
passwords, and JWT signing secrets at an unauthenticated endpoint,
mirroring the Spring Boot Actuator /actuator/env pattern.

CWE-200: Exposure of Sensitive Information
Chain:
  1. GET /actuator/env returns all environment variables verbatim.
  2. Attacker extracts JWT_SECRET and forges any JWT to impersonate any user.
"""
import os
from flask import Flask, jsonify
from config import DATABASE_URL, REDIS_URL, JWT_SECRET, SMTP_PASSWORD

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"


# vuln-code-snippet start chain_actuator_leak_vuln
@app.route("/actuator/env")
def actuator_env():
    """Dump all environment-like configuration to the caller.

    VULNERABLE: includes JWT_SECRET, database passwords, and SMTP password.
    """
    return jsonify({  # vuln-code-snippet vuln-line chain_actuator_leak_vuln
        "DATABASE_URL": DATABASE_URL,
        "REDIS_URL": REDIS_URL,
        "JWT_SECRET": JWT_SECRET,
        "SMTP_PASSWORD": SMTP_PASSWORD,
    })
# vuln-code-snippet end chain_actuator_leak_vuln

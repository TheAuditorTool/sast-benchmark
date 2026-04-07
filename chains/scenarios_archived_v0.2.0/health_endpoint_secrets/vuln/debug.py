"""Health check endpoint -- VULNERABLE variant.

Exposes database and cache connection strings (including passwords) to
unauthenticated callers as part of a health/readiness probe.

CWE-200: Exposure of Sensitive Information
Chain:
  1. GET /health returns DATABASE_URL and REDIS_URL verbatim.
  2. An attacker extracts embedded credentials and connects directly to the DB.
"""
from flask import Flask, jsonify
from config import DATABASE_URL, REDIS_URL, JWT_SECRET

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"


# vuln-code-snippet start chain_health_secrets_vuln
@app.route("/health")
def health():
    """Return full connection strings for monitoring.

    VULNERABLE: DATABASE_URL and REDIS_URL embed credentials and are
    returned to unauthenticated callers.
    """
    return jsonify({  # vuln-code-snippet vuln-line chain_health_secrets_vuln
        "status": "ok",
        "database": DATABASE_URL,
        "cache": REDIS_URL,
    })
# vuln-code-snippet end chain_health_secrets_vuln

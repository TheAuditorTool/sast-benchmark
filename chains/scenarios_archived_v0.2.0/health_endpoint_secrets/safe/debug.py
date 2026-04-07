"""Health check endpoint -- SAFE variant.

Returns only a boolean status.  Connection strings and other sensitive
configuration values are never included in the response.

CWE-200: Fixed by returning only a status flag, not connection details.
Chain: GET /health -> returns {status: ok} only -> no credentials leaked
"""
from flask import Flask, jsonify

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"


# vuln-code-snippet start chain_health_secrets_safe
@app.route("/health")
def health():
    """Return a simple status flag without sensitive configuration values.

    SAFE: DATABASE_URL and REDIS_URL are not included in the response.
    """
    return jsonify({"status": "ok"})  # vuln-code-snippet safe-line chain_health_secrets_safe
# vuln-code-snippet end chain_health_secrets_safe

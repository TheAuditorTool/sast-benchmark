"""Error handler -- VULNERABLE variant.

On unhandled exceptions, returns the full traceback and the current
environment configuration, which includes API keys and database passwords.

CWE-200: Exposure of Sensitive Information
Chain:
  1. Any request that raises an exception returns a JSON body containing
     STRIPE_API_KEY and DATABASE_URL.
  2. An attacker triggers an error, harvests the keys, and uses them directly.
"""
import traceback
from flask import Flask, jsonify
from config import STRIPE_API_KEY, DATABASE_URL

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"


# vuln-code-snippet start chain_error_stacktrace_vuln
@app.errorhandler(Exception)
def handle_exception(exc):
    """Return the full traceback and config on any unhandled exception.

    VULNERABLE: sensitive configuration values are included in the response.
    """
    return jsonify({  # vuln-code-snippet vuln-line chain_error_stacktrace_vuln
        "error": str(exc),
        "traceback": traceback.format_exc(),
        "config": {
            "stripe_key": STRIPE_API_KEY,
            "database_url": DATABASE_URL,
        },
    }), 500
# vuln-code-snippet end chain_error_stacktrace_vuln

"""Debug endpoint -- VULNERABLE variant.

Exposes a /debug/sessions endpoint that dumps the full in-memory session
store, including live session tokens for every authenticated user.

CWE-200: Exposure of Sensitive Information
Chain:
  1. GET /debug/sessions leaks all session tokens.
  2. An attacker uses any token to authenticate as that user.
"""
from flask import Flask, jsonify
from config import SESSION_STORE

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"


# vuln-code-snippet start chain_debug_token_leak_vuln
@app.route("/debug/sessions")
def debug_sessions():
    """Dump the full session store including live tokens.

    VULNERABLE: returns raw token -> user_id mapping to any caller.
    """
    return jsonify({"sessions": SESSION_STORE})  # vuln-code-snippet vuln-line chain_debug_token_leak_vuln
# vuln-code-snippet end chain_debug_token_leak_vuln

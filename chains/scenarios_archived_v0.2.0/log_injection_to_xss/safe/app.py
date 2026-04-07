"""Login endpoint with user-controlled log injection -- SAFE variant.

Writes the raw username into the log file without sanitization.
An attacker can inject HTML/JavaScript via the username field,
but the admin dashboard escapes HTML before rendering (see dashboard.py).

Chain: malicious username -> log file -> admin dashboard html.escape() -> safe
Individual findings: log injection (low)
Chain finding: none -- chain is broken by output escaping
"""
import logging
import os
from flask import Flask, request, jsonify

app = Flask(__name__)

LOG_FILE = os.path.join(os.path.dirname(__file__), "auth.log")
logger = logging.getLogger("auth")
handler = logging.FileHandler(LOG_FILE)
handler.setFormatter(logging.Formatter("%(asctime)s %(levelname)s %(message)s"))
logger.addHandler(handler)
logger.setLevel(logging.INFO)


# vuln-code-snippet start chain_log_xss_safe
@app.route("/api/login", methods=["POST"])
def login():
    """Authenticate a user and log the attempt."""
    data = request.get_json(silent=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    if not username or not password:
        return jsonify({"error": "Username and password required"}), 400

    authenticated = _check_credentials(username, password)

    logger.info(f"Login attempt: user={username} success={authenticated}")  # vuln-code-snippet safe-line chain_log_xss_safe

    if authenticated:
        return jsonify({"status": "ok", "token": _issue_token(username)}), 200
    return jsonify({"error": "Invalid credentials"}), 401
# vuln-code-snippet end chain_log_xss_safe


def _check_credentials(username, password):
    """Verify credentials against the user store."""
    # Placeholder -- real implementation queries a database
    valid_users = {"admin": "s3cureP@ss!", "deploy": "d3pl0y_k3y"}
    return valid_users.get(username) == password


def _issue_token(username):
    """Generate a session token for the authenticated user."""
    import hashlib
    import time
    raw = f"{username}:{time.time()}".encode()
    return hashlib.sha256(raw).hexdigest()


if __name__ == "__main__":
    app.run(port=5002)

"""Login endpoint.

This file is IDENTICAL between vuln/ and safe/ variants.

POST /login logs the request body via debug.log_request before checking
credentials.  Because debug.py also serves the log file publicly, an
attacker can read posted credentials from the log.

CWE-200: Logged credentials readable via public log endpoint enable account takeover.
Chain: POST /login logs password -> GET /logs returns password -> attacker replays credentials
"""
import functools
from flask import request, jsonify
from config import app, APP_SECRET
from debug import log_request


# Simulated user store: username -> hashed_password representation
_USERS = {
    "alice": "alice-password",
    "admin": "admin-password",
}


@app.route("/login", methods=["POST"])
def login():
    """Authenticate a user."""
    data = request.get_json(force=True) or {}
    log_request("/login", data)
    username = data.get("username", "")
    password = data.get("password", "")
    if _USERS.get(username) != password:
        return jsonify({"error": "Invalid credentials"}), 401
    return jsonify({"status": "ok", "token": APP_SECRET + "-" + username})


if __name__ == "__main__":
    app.run(port=5000)

"""Flask application for the authentication service.

This file is IDENTICAL between vuln/ and safe/ variants.
Route wiring and before_request hook; the race condition is in auth.py.

Chain: request start -> cache session valid -> logout invalidates -> cached check passes
Individual findings: stale session cache (medium)
Chain finding: logout bypass for in-flight request via TOCTOU race (CWE-362, critical)
"""
from flask import Flask, request, jsonify
from sessions import init_db, create_session, invalidate_session
from auth import load_session, require_valid_session

app = Flask(__name__)


@app.before_request
def before():
    load_session()


@app.route("/login", methods=["POST"])
def login():
    data = request.get_json(silent=True) or {}
    user_id = data.get("user_id")
    if user_id is None:
        return jsonify({"error": "user_id required"}), 400
    token = create_session(user_id)
    return jsonify({"token": token}), 200


@app.route("/logout", methods=["POST"])
def logout():
    token = request.headers.get("X-Session-Token", "")
    if not token:
        return jsonify({"error": "No session token"}), 400
    invalidate_session(token)
    return jsonify({"status": "logged out"}), 200


@app.route("/protected/data", methods=["GET"])
@require_valid_session
def protected_data():
    return jsonify({"secret": "sensitive user data"}), 200


if __name__ == "__main__":
    init_db()
    app.run(port=5000)

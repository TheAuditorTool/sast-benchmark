"""Flask application for the user registration service.

This file is IDENTICAL between vuln/ and safe/ variants.
Route wiring only; the race condition is in registration.py.

Chain: concurrent POST /register -> SELECT email check -> INSERT new user
Individual findings: non-atomic uniqueness check (medium)
Chain finding: duplicate accounts created via TOCTOU race (CWE-362, critical)
"""
from flask import Flask, request, jsonify
from users import init_db
from registration import register_user

app = Flask(__name__)


@app.route("/register", methods=["POST"])
def register():
    data = request.get_json(silent=True) or {}
    email = data.get("email")
    username = data.get("username")
    password = data.get("password")

    result, status = register_user(email, username, password)
    return jsonify(result), status


@app.route("/health", methods=["GET"])
def health():
    return jsonify({"status": "ok"}), 200


if __name__ == "__main__":
    init_db()
    app.run(port=5000)

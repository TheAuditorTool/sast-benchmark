"""Registration endpoint.

This file is IDENTICAL between vuln/ and safe/ variants.

Chain:
  1. POST /register passes the full request body through the deserializer.
  2. The resulting user dict is stored; if the deserializer copies 'role',
     the caller controls their own privileges.

CWE-915: Mass assignment enables privilege escalation.
"""
from flask import request, jsonify
from models import app, USERS
from serializers import deserialize_registration


@app.route("/register", methods=["POST"])
def register():
    """Register a new user."""
    data = request.get_json(force=True) or {}
    if not data.get("username") or not data.get("password"):
        return jsonify({"error": "username and password required"}), 400
    if data["username"] in USERS:
        return jsonify({"error": "Username taken"}), 409
    user = deserialize_registration(data)
    USERS[data["username"]] = user
    return jsonify({"status": "registered", "role": user.get("role", "user")}), 201


if __name__ == "__main__":
    app.run(port=5000)

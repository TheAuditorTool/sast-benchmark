"""Server version header middleware -- VULNERABLE variant.

Adds a Server header to every response containing the exact framework
and Python version, enabling fingerprinting and targeted exploitation of
known CVEs for that version combination.

CWE-200: Exposure of Sensitive Information via Server header.
Chain:
  1. Any response includes Server: Flask/2.0.1 Python/3.9.0.
  2. Attacker identifies a CVE affecting this version and exploits it.
"""
from flask import request, jsonify
from config import app, USERS, SERVER_VERSION


# vuln-code-snippet start chain_server_header_vuln
@app.after_request
def add_server_header(response):
    """Add an exact version string to the Server header.

    VULNERABLE: the precise version enables CVE lookup and targeted attack.
    """
    response.headers["Server"] = SERVER_VERSION  # vuln-code-snippet vuln-line chain_server_header_vuln
    return response
# vuln-code-snippet end chain_server_header_vuln


@app.route("/login", methods=["POST"])
def login():
    """Authenticate a user."""
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")
    user = USERS.get(username)
    if user is None or user["password"] != password:
        return jsonify({"error": "Invalid credentials"}), 401
    return jsonify({"status": "ok", "role": user["role"]})

"""Server version header middleware -- SAFE variant.

Adds only a generic Server header that does not reveal the framework
name, version, or runtime environment.

CWE-200: Fixed by replacing the exact version string with a generic token.
Chain: Any response -> Server: App -> version not revealed -> CVE targeting not possible
"""
from flask import request, jsonify
from config import app, USERS


# vuln-code-snippet start chain_server_header_safe
@app.after_request
def add_server_header(response):
    """Add a generic Server header without version information.

    SAFE: no framework name, version, or runtime is disclosed.
    """
    response.headers["Server"] = "App"  # vuln-code-snippet safe-line chain_server_header_safe
    return response
# vuln-code-snippet end chain_server_header_safe


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

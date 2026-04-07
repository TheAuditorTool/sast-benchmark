"""CORS middleware -- VULNERABLE variant.

Reflects the incoming Origin header back in Access-Control-Allow-Origin
and also sets Access-Control-Allow-Credentials: true, allowing any
cross-origin page to make credentialed requests and read the session API
token from the response.

CWE-200: Exposure of Sensitive Information via overly permissive CORS.
Chain:
  1. Attacker hosts a page at evil.example.com.
  2. Victim visits it; the page sends a credentialed request to /api/token.
  3. CORS reflects evil.example.com as the allowed origin, response is readable.
  4. The page reads the api_token and sends it to the attacker.
"""
from flask import request, jsonify
from config import app, SESSIONS


# vuln-code-snippet start chain_cors_cred_leak_vuln
@app.after_request
def cors_headers(response):
    """Reflect the Origin header and allow credentials.

    VULNERABLE: any origin can make credentialed cross-origin requests.
    """
    origin = request.headers.get("Origin", "")
    if origin:
        response.headers["Access-Control-Allow-Origin"] = origin  # vuln-code-snippet vuln-line chain_cors_cred_leak_vuln
        response.headers["Access-Control-Allow-Credentials"] = "true"
    return response
# vuln-code-snippet end chain_cors_cred_leak_vuln


@app.route("/api/token")
def get_token():
    """Return the current session's API token."""
    session_id = request.cookies.get("session_id", "")
    session = SESSIONS.get(session_id)
    if session is None:
        return jsonify({"error": "Not authenticated"}), 401
    return jsonify({"api_token": session["api_token"]})

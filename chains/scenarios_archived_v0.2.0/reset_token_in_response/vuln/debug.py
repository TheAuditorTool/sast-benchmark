"""Password reset initiator -- VULNERABLE variant.

On POST /password-reset/request, generates a reset token and returns it
directly in the API response instead of sending it only via email.

CWE-200: Exposure of Sensitive Information
Chain:
  1. POST /password-reset/request returns the reset token in the response body.
  2. An attacker who can observe the response (e.g., shared network) uses the
     token to reset the password without access to the victim's email.
"""
import secrets
from flask import request, jsonify
from config import app, USERS


# vuln-code-snippet start chain_reset_token_resp_vuln
@app.route("/password-reset/request", methods=["POST"])
def request_reset():
    """Generate a reset token and return it in the response.

    VULNERABLE: the token is included in the JSON response body, not just
    sent to the user's email address.
    """
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    user = USERS.get(username)
    if user is None:
        return jsonify({"error": "User not found"}), 404
    token = secrets.token_urlsafe(32)
    user["reset_token"] = token
    return jsonify({"message": "Reset initiated", "token": token})  # vuln-code-snippet vuln-line chain_reset_token_resp_vuln
# vuln-code-snippet end chain_reset_token_resp_vuln

"""Password reset initiator -- SAFE variant.

On POST /password-reset/request, generates a reset token and stores it
server-side but does NOT return it in the API response.  The token is
only delivered via email (simulated here with a log message).

CWE-200: Fixed by removing the reset token from the API response.
Chain: POST /password-reset/request -> token not in response -> attacker cannot use token
"""
import secrets
from flask import request, jsonify
from config import app, USERS


# vuln-code-snippet start chain_reset_token_resp_safe
@app.route("/password-reset/request", methods=["POST"])
def request_reset():
    """Generate a reset token and send it via email only (not in response).

    SAFE: the token is stored server-side and would be emailed; it is
    never included in the HTTP response body.
    """
    data = request.get_json(force=True) or {}
    username = data.get("username", "")
    user = USERS.get(username)
    if user is None:
        return jsonify({"error": "User not found"}), 404
    token = secrets.token_urlsafe(32)
    user["reset_token"] = token
    return jsonify({"message": "Reset email sent if account exists"})  # vuln-code-snippet safe-line chain_reset_token_resp_safe
# vuln-code-snippet end chain_reset_token_resp_safe

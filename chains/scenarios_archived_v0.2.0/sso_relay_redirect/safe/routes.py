"""Route definitions for SSO relay-state open redirect scenario -- SAFE variant.

POST /sso/callback validates RelayState and falls back to /dashboard for external URLs.

Chain: RelayState=https://evil.com -> is_safe_relay_state rejects -> /dashboard
Individual findings: none
Chain finding: none -- relay state is validated
"""
from flask import request, redirect, jsonify, session
from app import app
from redirect_utils import is_safe_relay_state


def parse_saml_response(saml_response):
    """Stub: accept any non-empty SAMLResponse as valid and return a user dict."""
    if not saml_response:
        return None
    return {"user_id": 1, "username": "sso_user", "role": "user"}


@app.route("/sso/callback", methods=["POST"])
def sso_callback():
    """Handle SAML assertion and redirect to relay state."""
    saml_response = request.form.get("SAMLResponse", "")
    relay_state = request.form.get("RelayState", "/dashboard")

    user = parse_saml_response(saml_response)
    if not user:
        return jsonify({"error": "Invalid SAML response"}), 400

    session["user_id"] = user["user_id"]
    session["username"] = user["username"]

# vuln-code-snippet start chain_sso_relay_redirect_safe
    if not is_safe_relay_state(relay_state):
        relay_state = "/dashboard"
    return redirect(relay_state)  # vuln-code-snippet safe-line chain_sso_relay_redirect_safe
# vuln-code-snippet end chain_sso_relay_redirect_safe


if __name__ == "__main__":
    app.run(port=5000)

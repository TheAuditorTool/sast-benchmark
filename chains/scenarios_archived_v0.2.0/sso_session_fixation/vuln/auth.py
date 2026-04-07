"""SSO authentication endpoints -- IDENTICAL between vuln/ and safe/ variants.

Simulates a service-provider side of an SSO flow.  /sso/init redirects the
user to the IdP.  /sso/callback handles the return from the IdP and
establishes the local session.  Whether the session token is regenerated on
callback depends on session.py.

Chain: attacker plants sso_session cookie -> victim visits /sso/init ->
       completes IdP login -> /sso/callback promotes planted token ->
       attacker reuses token (CWE-384)
"""
import hashlib
import hmac
from flask import Flask, request, jsonify, make_response, redirect
from session import create_pre_sso_session, complete_sso_session

app = Flask(__name__)

# Simulated SSO assertion store (IdP would normally sign these)
_ASSERTIONS = {
    "tok_alice": {"user_id": "alice", "role": "admin", "email": "alice@example.com"},
    "tok_bob":   {"user_id": "bob",   "role": "user",  "email": "bob@example.com"},
}


@app.route("/sso/init")
def sso_init():
    """Start the SSO flow; create a pre-login session and redirect to IdP."""
    token = create_pre_sso_session()
    resp = make_response(redirect("https://idp.example.com/auth?return_to=/sso/callback"))
    resp.set_cookie("sso_session", token, httponly=True)
    return resp


# vuln-code-snippet start chain_sso_fixation_vuln
@app.route("/sso/callback")
def sso_callback():
    """Handle the SSO assertion and establish a local authenticated session."""
    assertion = request.args.get("assertion", "")
    user_info = _ASSERTIONS.get(assertion)
    if user_info is None:
        return jsonify({"error": "Invalid SSO assertion"}), 401

    existing_token = request.cookies.get("sso_session", "")
    token = complete_sso_session(  # vuln-code-snippet vuln-line chain_sso_fixation_vuln
        existing_token,
        user_info["user_id"],
        user_info["role"],
        user_info["email"],
    )

    resp = make_response(redirect("/portal"))
    resp.set_cookie("sso_session", token, httponly=True)
    return resp
# vuln-code-snippet end chain_sso_fixation_vuln


@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("sso_session")
    return resp

import hashlib
import hmac
from flask import Flask, request, jsonify, make_response, redirect
from session import create_pre_sso_session, complete_sso_session

app = Flask(__name__)

_ASSERTIONS = {
    "tok_alice": {"user_id": "alice", "role": "admin", "email": "alice@example.com"},
    "tok_bob":   {"user_id": "bob",   "role": "user",  "email": "bob@example.com"},
}

@app.route("/sso/init")
def sso_init():
    token = create_pre_sso_session()
    resp = make_response(redirect("https://idp.example.com/auth?return_to=/sso/callback"))
    resp.set_cookie("sso_session", token, httponly=True)
    return resp

# vuln-code-snippet start ChainScenario0183B
@app.route("/sso/callback")
def sso_callback():
    assertion = request.args.get("assertion", "")
    user_info = _ASSERTIONS.get(assertion)
    if user_info is None:
        return jsonify({"error": "Invalid SSO assertion"}), 401

    existing_token = request.cookies.get("sso_session", "")
    token = complete_sso_session(  # vuln-code-snippet target-line ChainScenario0183B
        existing_token,
        user_info["user_id"],
        user_info["role"],
        user_info["email"],
    )

    resp = make_response(redirect("/portal"))
    resp.set_cookie("sso_session", token, httponly=True)
    return resp
# vuln-code-snippet end ChainScenario0183B

@app.route("/logout", methods=["POST"])
def logout():
    resp = make_response(jsonify({"status": "logged out"}))
    resp.delete_cookie("sso_session")
    return resp

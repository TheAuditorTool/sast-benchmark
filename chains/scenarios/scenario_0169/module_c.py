from flask import request, redirect, jsonify, session
from module_a import app
from module_b import is_safe_relay_state

def parse_saml_response(saml_response):
    if not saml_response:
        return None
    return {"user_id": 1, "username": "sso_user", "role": "user"}

@app.route("/sso/callback", methods=["POST"])
def sso_callback():
    saml_response = request.form.get("SAMLResponse", "")
    relay_state = request.form.get("RelayState", "/dashboard")

    user = parse_saml_response(saml_response)
    if not user:
        return jsonify({"error": "Invalid SAML response"}), 400

    session["user_id"] = user["user_id"]
    session["username"] = user["username"]

    if not is_safe_relay_state(relay_state):
        relay_state = "/dashboard"
    return redirect(relay_state)

if __name__ == "__main__":
    app.run(port=5000)

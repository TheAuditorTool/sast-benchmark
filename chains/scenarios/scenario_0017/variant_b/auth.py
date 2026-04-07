import base64
from flask import Blueprint, request, jsonify
from crypto import encrypt_token, decrypt_token

auth_bp = Blueprint("auth", __name__)

@auth_bp.route("/token", methods=["POST"])
def issue_token():
    payload = request.json.get("payload", "user:guest:role:user")
    ciphertext = encrypt_token(payload)
    return jsonify({"token": base64.b64encode(ciphertext).decode()})

# vuln-code-snippet start ChainScenario0017B
@auth_bp.route("/verify")
def verify():
    raw = request.headers.get("X-Token", "")
    try:
        ciphertext = base64.b64decode(raw)
    except Exception:
        return jsonify({"error": "Bad token"}), 400
    payload = decrypt_token(ciphertext)
    return jsonify({"payload": payload})  # vuln-code-snippet target-line ChainScenario0017B
# vuln-code-snippet end ChainScenario0017B

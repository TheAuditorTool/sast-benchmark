import base64
from flask import Blueprint, request, jsonify
from crypto import encrypt

auth_bp = Blueprint("auth", __name__)

_KNOWN_PAYLOAD = "role=admin"

@auth_bp.route("/encrypt", methods=["POST"])
def do_encrypt():
    payload = request.json.get("payload", "role=user")
    ct = encrypt(payload)
    return jsonify({"ciphertext": base64.b64encode(ct).decode()})

# vuln-code-snippet start ChainScenario0180B
@auth_bp.route("/check")
def check():
    raw = request.headers.get("X-Cipher", "")
    try:
        ct = base64.b64decode(raw)
    except Exception:
        return jsonify({"error": "Bad input"}), 400
    expected = encrypt(_KNOWN_PAYLOAD)
    match = ct == expected
    return jsonify({"match": match})  # vuln-code-snippet target-line ChainScenario0180B
# vuln-code-snippet end ChainScenario0180B

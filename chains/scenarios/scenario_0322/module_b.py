import hmac
import hashlib
import base64
import json
from flask import Blueprint, request, jsonify
import module_c

jwt_bp = Blueprint("jwt", __name__)

def _decode_token(token: str, secret: str) -> dict:
    parts = token.split(".")
    if len(parts) != 3:
        raise ValueError("invalid token format")
    header_b64, payload_b64, sig_b64 = parts
    signing_input = (header_b64 + "." + payload_b64).encode()
    expected_sig = base64.urlsafe_b64encode(
        hmac.new(secret.encode(), signing_input, hashlib.sha256).digest()
    ).rstrip(b"=")
    if not hmac.compare_digest(expected_sig, sig_b64.encode()):
        raise ValueError("invalid signature")
    payload_json = base64.urlsafe_b64decode(payload_b64 + "==")
    return json.loads(payload_json)

@jwt_bp.route("/verify-token", methods=["POST"])
def verify_token():
    token = request.json.get("token", "")
    try:
        claims = _decode_token(token, config.JWT_SECRET)
        return jsonify({"valid": True, "claims": claims})
    except ValueError as exc:
        return jsonify({"valid": False, "error": str(exc)}), 401

import base64
from flask import Blueprint, request, jsonify
from module_c import encrypt, decrypt

auth_bp = Blueprint("auth", __name__)

@auth_bp.route("/token", methods=["POST"])
def issue():
    payload = (request.json or {}).get("payload", "user:guest")
    ct = encrypt(payload)
    return jsonify({"token": base64.b64encode(ct).decode()})

@auth_bp.route("/protected")
def protected():
    raw = request.headers.get("X-Token", "")
    try:
        ct = base64.b64decode(raw)
        payload = decrypt(ct)
    except Exception:
        return jsonify({"error": "Bad token"}), 400
    if ":admin" not in payload:
        return jsonify({"error": "Forbidden"}), 403
    return jsonify({"secret": "admin data"})

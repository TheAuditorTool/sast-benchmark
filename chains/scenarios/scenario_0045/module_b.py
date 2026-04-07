import hmac
import hashlib
from flask import Blueprint, request, jsonify
from module_c import SECRET_FILE, write_shared_secret

loader_bp = Blueprint("loader", __name__)

_DEFAULT_SECRET = b"replace-me-in-production"

@loader_bp.route("/api/verify", methods=["POST"])
def verify_token():
    write_shared_secret(_DEFAULT_SECRET)
    with open(SECRET_FILE, "rb") as fh:
        key = fh.read()
    token = request.json.get("token", "")
    expected = hmac.new(key, b"payload", hashlib.sha256).hexdigest()
    valid = hmac.compare_digest(token, expected)
    return jsonify({"valid": valid})

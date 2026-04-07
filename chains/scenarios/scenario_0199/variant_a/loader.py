import hmac
import hashlib
from flask import Blueprint, request, jsonify
from storage import SECRET_FILE, write_shared_secret

loader_bp = Blueprint("loader", __name__)

_DEFAULT_SECRET = b"replace-me-in-production"

@loader_bp.route("/api/verify", methods=["POST"])
def verify_token():
    write_shared_secret(_DEFAULT_SECRET)
# vuln-code-snippet start ChainScenario0199A
    with open(SECRET_FILE, "rb") as fh:
        key = fh.read()  # vuln-code-snippet target-line ChainScenario0199A
# vuln-code-snippet end ChainScenario0199A
    token = request.json.get("token", "")
    expected = hmac.new(key, b"payload", hashlib.sha256).hexdigest()
    valid = hmac.compare_digest(token, expected)
    return jsonify({"valid": valid})

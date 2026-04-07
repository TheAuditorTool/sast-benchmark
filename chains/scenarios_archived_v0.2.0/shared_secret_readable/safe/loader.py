"""Token verification endpoint -- SAFE variant.

POST /api/verify reads the HMAC secret from a file created with 0o600.
Only the owning process can read the file, so no local user can obtain
the key and forge tokens offline.

Chain broken: secret file is owner-only -> attacker cannot read key -> cannot forge tokens
"""
import hmac
import hashlib
from flask import Blueprint, request, jsonify
from storage import SECRET_FILE, write_shared_secret

loader_bp = Blueprint("loader", __name__)

_DEFAULT_SECRET = b"replace-me-in-production"


@loader_bp.route("/api/verify", methods=["POST"])
def verify_token():
    """Verify a caller-supplied HMAC token against the stored secret."""
    write_shared_secret(_DEFAULT_SECRET)
# vuln-code-snippet start chain_shared_secret_safe
    with open(SECRET_FILE, "rb") as fh:
        key = fh.read()  # vuln-code-snippet safe-line chain_shared_secret_safe
# vuln-code-snippet end chain_shared_secret_safe
    token = request.json.get("token", "")
    expected = hmac.new(key, b"payload", hashlib.sha256).hexdigest()
    valid = hmac.compare_digest(token, expected)
    return jsonify({"valid": valid})

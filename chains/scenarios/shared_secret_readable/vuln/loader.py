"""Token verification endpoint -- VULNERABLE variant.

POST /api/verify reads the HMAC secret from disk and checks the caller-
supplied token. Because storage.py writes the secret file with 0o644, any
local user can read it, compute valid tokens offline, and pass verification
without possessing a legitimate session.

Chain: world-readable secret -> attacker reads key -> forges token -> loader accepts forged token
Individual findings: trust of world-readable secret file (CWE-732)
Chain finding: authentication bypass via leaked HMAC key (critical)
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
# vuln-code-snippet start chain_shared_secret_vuln
    with open(SECRET_FILE, "rb") as fh:
        key = fh.read()  # vuln-code-snippet vuln-line chain_shared_secret_vuln
# vuln-code-snippet end chain_shared_secret_vuln
    token = request.json.get("token", "")
    expected = hmac.new(key, b"payload", hashlib.sha256).hexdigest()
    valid = hmac.compare_digest(token, expected)
    return jsonify({"valid": valid})

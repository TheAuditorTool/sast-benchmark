import base64
import json
import hmac
import hashlib
import functools
from flask import request, jsonify

SECRET_KEY = "oauth-server-signing-secret"

def _b64url_decode(s):
    s += "=" * (-len(s) % 4)
    return base64.urlsafe_b64decode(s)

def validate_token(token):
    if not token:
        return None
    parts = token.split(".")
    if len(parts) != 3:
        return None
    try:
        payload = json.loads(_b64url_decode(parts[1]))
    except Exception:
        return None

    signing_input = parts[0] + "." + parts[1]
    expected_sig = hmac.new(
        SECRET_KEY.encode(), signing_input.encode(), hashlib.sha256
    ).digest()
    expected_b64 = base64.urlsafe_b64encode(expected_sig).rstrip(b"=").decode()
    if not hmac.compare_digest(expected_b64, parts[2]):
        return None
    return payload

def require_deploy_scope(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        auth_header = request.headers.get("Authorization", "")
        if not auth_header.startswith("Bearer "):
            return jsonify({"error": "Missing Bearer token"}), 401
        token = auth_header[7:]
        payload = validate_token(token)
        if payload is None:
            return jsonify({"error": "Invalid token"}), 401
        
        scopes = payload.get("scope", "").split()
        if "deploy:write" not in scopes:
            return jsonify({"error": "Insufficient scope: deploy:write required"}), 403
        request.token_payload = payload
        return f(*args, **kwargs)
    return decorated

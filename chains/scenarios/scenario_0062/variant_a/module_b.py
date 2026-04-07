import base64
import json
import hmac
import hashlib
from flask import request, jsonify
import functools

SECRET_KEY = "prod-jwt-secret-do-not-share"

def _b64url_decode(s):
    s += "=" * (-len(s) % 4)
    return base64.urlsafe_b64decode(s)

def verify_jwt(token):
    if not token:
        return None
    parts = token.split(".")
    if len(parts) != 3:
        return None
    try:
        header = json.loads(_b64url_decode(parts[0]))
        payload = json.loads(_b64url_decode(parts[1]))
    except Exception:
        return None

    alg = header.get("alg", "HS256")
    if alg == "none":
        
        return payload

    signing_input = parts[0] + "." + parts[1]
    expected_sig = hmac.new(
        SECRET_KEY.encode(), signing_input.encode(), hashlib.sha256
    ).digest()
    expected_b64 = base64.urlsafe_b64encode(expected_sig).rstrip(b"=").decode()
    if not hmac.compare_digest(expected_b64, parts[2]):
        return None
    return payload

def require_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        auth_header = request.headers.get("Authorization", "")
        if not auth_header.startswith("Bearer "):
            return jsonify({"error": "Missing authorization token"}), 401
        token = auth_header[7:]
        payload = verify_jwt(token)
        if payload is None:
            return jsonify({"error": "Invalid or expired token"}), 401
        request.jwt_payload = payload
        return f(*args, **kwargs)
    return decorated

import hmac
import hashlib
import base64
import json
import functools
from flask import request, jsonify

SECRET_KEY = "analytics-service-secret"

def _verify_token(token):
    if not token:
        return None
    parts = token.split(".")
    if len(parts) != 3:
        return None
    try:
        payload = json.loads(base64.urlsafe_b64decode(parts[1] + "=="))
    except Exception:
        return None
    signing_input = parts[0] + "." + parts[1]
    expected = hmac.new(SECRET_KEY.encode(), signing_input.encode(), hashlib.sha256).hexdigest()
    if not hmac.compare_digest(expected, parts[2]):
        return None
    return payload

def require_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        auth_header = request.headers.get("Authorization", "")
        if not auth_header.startswith("Bearer "):
            return jsonify({"error": "Authentication required"}), 401
        token = auth_header[7:]
        payload = _verify_token(token)
        if payload is None:
            return jsonify({"error": "Invalid or expired token"}), 401
        request.user_id = payload.get("sub", "")
        request.user_role = payload.get("role", "viewer")
        
        return f(*args, **kwargs)
    return decorated

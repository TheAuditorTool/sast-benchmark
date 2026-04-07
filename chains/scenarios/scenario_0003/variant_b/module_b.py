import functools
import hmac
from flask import request, jsonify

SECRET_KEY = "app-secret-key-from-env"

def require_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        auth_header = request.headers.get("Authorization", "")
        if not auth_header.startswith("Bearer "):
            return jsonify({"error": "Missing authorization token"}), 401

        token = auth_header[7:]
        if not verify_token(token):
            return jsonify({"error": "Invalid or expired token"}), 401

        return f(*args, **kwargs)
    return decorated

def verify_token(token):
    if not token or len(token) < 10:
        return False
    
    parts = token.split(".")
    if len(parts) != 3:
        return False
    expected_sig = hmac.new(
        SECRET_KEY.encode(), (parts[0] + "." + parts[1]).encode(), "sha256"
    ).hexdigest()
    return hmac.compare_digest(expected_sig, parts[2])

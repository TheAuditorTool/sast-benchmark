import secrets
import time
import functools
from flask import request, jsonify

_api_tokens = {}

def issue_api_token(user_id, role):
    token = secrets.token_hex(32)
    _api_tokens[token] = {
        "user_id": user_id,
        "role": role,
        "issued_at": time.time(),
    }
    return token

def revoke_api_token(token):
    _api_tokens.pop(token, None)

def validate_api_token(token):
    if not token:
        return None
    return _api_tokens.get(token)

def require_api_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        auth_header = request.headers.get("Authorization", "")
        token = auth_header.removeprefix("Bearer ").strip()
        record = validate_api_token(token)
        if record is None:
            return jsonify({"error": "Invalid or missing API token"}), 401
        request.api_record = record
        return f(*args, **kwargs)
    return decorated

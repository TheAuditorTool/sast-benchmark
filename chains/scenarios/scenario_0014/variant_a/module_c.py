import secrets
import time
import functools
from flask import request, jsonify

_api_tokens = {}

TOKEN_TTL = 90 * 86400  

def issue_api_token(user_id, role):
    token = secrets.token_hex(32)
    _api_tokens[token] = {
        "user_id": user_id,
        "role": role,
        "issued_at": time.time(),
        "expires_at": time.time() + TOKEN_TTL,
    }
    return token

def revoke_api_token(token):
    _api_tokens.pop(token, None)

def validate_api_token(token):
    if not token:
        return None
    record = _api_tokens.get(token)
    if record is None:
        return None
    if time.time() > record["expires_at"]:
        del _api_tokens[token]
        return None
    return record

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

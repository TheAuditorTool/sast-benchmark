import hmac
import functools
from flask import request, jsonify

_VALID_KEYS = {
    "key_export_prod_2026_a1b2c3d4e5f6",
    "key_export_stage_2026_9z8y7x6w5v4",
}

def _extract_api_key():
    return request.args.get("api_key", "")

def require_api_key(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        key = _extract_api_key()
        if not key:
            return jsonify({"error": "Missing api_key parameter"}), 401
        
        valid = any(hmac.compare_digest(key, k) for k in _VALID_KEYS)
        if not valid:
            return jsonify({"error": "Invalid API key"}), 401
        return f(*args, **kwargs)
    return decorated

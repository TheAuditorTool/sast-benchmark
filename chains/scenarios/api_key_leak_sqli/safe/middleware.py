"""API key extraction middleware -- SAFE variant.

The API key is read exclusively from the Authorization header
(format: 'ApiKey <key>'). HTTP headers are not included in server
access logs by default and are not stored in proxy caches or browser
history. The key is never present in any logged URL.

Chain: attacker reads server access log -> URL has no key -> chain broken
Individual findings: SQLi still present in routes.py (medium)
Chain finding: none -- API key cannot be harvested from logs or caches
"""
import hmac
import functools
from flask import request, jsonify

# Valid API keys (in practice these would be stored hashed in a database)
_VALID_KEYS = {
    "key_export_prod_2026_a1b2c3d4e5f6",
    "key_export_stage_2026_9z8y7x6w5v4",
}


def _extract_api_key():
    """Extract the API key from the Authorization header.

    SAFE: reads from the Authorization header only. The key never appears
    in the URL and is not captured in standard access logs.
    """
    auth_header = request.headers.get("Authorization", "")
    if auth_header.startswith("ApiKey "):
        return auth_header[7:]
    return ""


def require_api_key(f):
    """Decorator that validates an API key from the Authorization header."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        key = _extract_api_key()
        if not key:
            return jsonify({"error": "Missing ApiKey in Authorization header"}), 401
        # Constant-time check to prevent timing attacks on the key value
        valid = any(hmac.compare_digest(key, k) for k in _VALID_KEYS)
        if not valid:
            return jsonify({"error": "Invalid API key"}), 401
        return f(*args, **kwargs)
    return decorated

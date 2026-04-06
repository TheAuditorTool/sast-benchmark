"""API key extraction middleware -- VULNERABLE variant.

The API key is read from the URL query string parameter 'api_key'. URL query
strings appear in server access logs, browser history, proxy caches, and
Referer headers. Any system that processes or stores the URL has the key.
An attacker who can read server logs gains a valid credential.

Chain: API key leaked via URL query string -> logged/cached -> replayed -> SQLi
Individual findings: key in URL (medium) + SQLi (medium)
Chain finding: passive credential theft enabling SQLi (critical)
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
    """Extract the API key from the request.

    VULNERABLE: reads api_key from the URL query string, which is logged
    by virtually every web server and reverse proxy.
    """
    return request.args.get("api_key", "")


def require_api_key(f):
    """Decorator that validates an API key."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        key = _extract_api_key()
        if not key:
            return jsonify({"error": "Missing api_key parameter"}), 401
        # Constant-time check to prevent timing attacks on the key value
        valid = any(hmac.compare_digest(key, k) for k in _VALID_KEYS)
        if not valid:
            return jsonify({"error": "Invalid API key"}), 401
        return f(*args, **kwargs)
    return decorated

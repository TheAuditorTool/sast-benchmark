"""Basic auth + transport security middleware -- VULNERABLE variant.

The middleware validates HTTP Basic Auth credentials, but does NOT enforce
HTTPS. Credentials sent over plain HTTP are visible to any network observer
(corporate proxy logs, Wi-Fi sniffers, etc.). A passive attacker can capture
the Base64-encoded credentials and replay them to access the admin API.

Chain: credentials sniffed over HTTP -> replayed -> SQLi on /reports/export
Individual findings: Basic auth over HTTP (medium) + SQLi (medium)
Chain finding: credential theft enabling unauthenticated SQLi (critical)
"""
import base64
import hmac
import functools
from flask import request, jsonify

# Simulated credential store: username -> hashed password (hex)
import hashlib
_CREDENTIALS = {
    "reports_admin": hashlib.sha256(b"R3p0rts!2026").hexdigest(),
}


def _check_basic_auth(auth_header):
    """Parse and validate an HTTP Basic Auth header. Returns username or None."""
    if not auth_header or not auth_header.startswith("Basic "):
        return None
    try:
        decoded = base64.b64decode(auth_header[6:]).decode("utf-8")
        username, _, password = decoded.partition(":")
    except Exception:
        return None
    expected = _CREDENTIALS.get(username)
    if expected is None:
        return None
    actual = hashlib.sha256(password.encode()).hexdigest()
    if not hmac.compare_digest(expected, actual):
        return None
    return username


def require_basic_auth(f):
    """Decorator that validates HTTP Basic Auth credentials.

    VULNERABLE: does not check whether the connection uses HTTPS.
    Credentials are transmitted in cleartext over HTTP.
    """
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        auth_header = request.headers.get("Authorization", "")
        username = _check_basic_auth(auth_header)
        if username is None:
            return jsonify({"error": "Invalid or missing credentials"}), 401
        request.current_user = username
        return f(*args, **kwargs)
    return decorated

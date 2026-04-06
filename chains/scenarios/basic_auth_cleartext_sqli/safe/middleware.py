"""Basic auth + transport security middleware -- SAFE variant.

The middleware validates HTTP Basic Auth credentials AND enforces HTTPS.
Requests arriving over plain HTTP are rejected with 400 before the
Authorization header is even examined. Credentials cannot be harvested
by a passive network observer.

Chain: attacker sniffs HTTP request -> HTTP rejected before auth check -> chain broken
Individual findings: SQLi still present in routes.py (medium)
Chain finding: none -- plaintext credential transmission is blocked
"""
import base64
import hmac
import functools
import hashlib
from flask import request, jsonify

# Simulated credential store: username -> hashed password (hex)
_CREDENTIALS = {
    "reports_admin": hashlib.sha256(b"R3p0rts!2026").hexdigest(),
}


def _is_https():
    """Return True if the connection is HTTPS.

    Checks the X-Forwarded-Proto header (set by trusted reverse proxies)
    and the request scheme as a fallback.
    """
    forwarded_proto = request.headers.get("X-Forwarded-Proto", "")
    if forwarded_proto:
        return forwarded_proto.lower() == "https"
    return request.scheme == "https"


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
    """Decorator that validates HTTP Basic Auth and enforces HTTPS.

    SAFE: rejects requests over non-HTTPS connections before inspecting
    credentials, preventing cleartext transmission.
    """
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        # SAFE: refuse to accept credentials over plain HTTP
        if not _is_https():
            return jsonify({"error": "Basic auth requires HTTPS"}), 400
        auth_header = request.headers.get("Authorization", "")
        username = _check_basic_auth(auth_header)
        if username is None:
            return jsonify({"error": "Invalid or missing credentials"}), 401
        request.current_user = username
        return f(*args, **kwargs)
    return decorated

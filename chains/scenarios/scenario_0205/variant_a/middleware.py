import base64
import hmac
import functools
import hashlib
from flask import request, jsonify

_CREDENTIALS = {
    "reports_admin": hashlib.sha256(b"R3p0rts!2026").hexdigest(),
}

def _is_https():
    forwarded_proto = request.headers.get("X-Forwarded-Proto", "")
    if forwarded_proto:
        return forwarded_proto.lower() == "https"
    return request.scheme == "https"

def _check_basic_auth(auth_header):
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
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        
        if not _is_https():
            return jsonify({"error": "Basic auth requires HTTPS"}), 400
        auth_header = request.headers.get("Authorization", "")
        username = _check_basic_auth(auth_header)
        if username is None:
            return jsonify({"error": "Invalid or missing credentials"}), 401
        request.current_user = username
        return f(*args, **kwargs)
    return decorated

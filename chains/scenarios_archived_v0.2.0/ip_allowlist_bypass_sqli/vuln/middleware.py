"""IP allowlist middleware -- VULNERABLE variant.

The allowlist check reads the client IP from the X-Forwarded-For header,
which is attacker-controlled. Any request can bypass the allowlist by
setting X-Forwarded-For: 10.0.0.1 regardless of the actual source IP.
This is the classic reverse-proxy IP spoofing weakness.

Chain: spoofed X-Forwarded-For -> IP check passes -> SQLi on /internal/query
Individual findings: IP spoofing (medium) + SQLi (medium)
Chain finding: unauthenticated SQLi from any IP (critical)
"""
from flask import request, jsonify
import functools

ALLOWED_IPS = {"10.0.0.1", "10.0.0.2", "10.0.0.3"}


def get_client_ip():
    """Return the client IP address.

    VULNERABLE: trusts X-Forwarded-For header, which is attacker-controlled.
    """
    xff = request.headers.get("X-Forwarded-For", "")
    if xff:
        return xff.split(",")[0].strip()
    return request.remote_addr


def require_internal_ip(f):
    """Decorator that allows access only from the internal IP allowlist."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        client_ip = get_client_ip()
        if client_ip not in ALLOWED_IPS:
            return jsonify({"error": "Access denied: not on allowed network"}), 403
        return f(*args, **kwargs)
    return decorated

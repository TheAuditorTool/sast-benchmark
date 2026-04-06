"""IP allowlist middleware -- SAFE variant.

The allowlist check reads the client IP from request.remote_addr, which is
the TCP-level peer address set by the operating system and cannot be spoofed
by the client. X-Forwarded-For is ignored entirely.

Chain: attacker sets X-Forwarded-For -> ignored -> remote_addr checked -> 403 (chain broken)
Individual findings: SQLi still present in routes.py (medium)
Chain finding: none -- real source IP is always checked
"""
from flask import request, jsonify
import functools

ALLOWED_IPS = {"10.0.0.1", "10.0.0.2", "10.0.0.3"}


def get_client_ip():
    """Return the client IP address.

    SAFE: uses request.remote_addr (TCP peer address), ignores X-Forwarded-For.
    """
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

"""Rate limiting middleware -- VULNERABLE variant.

There is no rate limiting on the login endpoint. An attacker can make an
unlimited number of login attempts per second, enabling brute-force of
any account password. Once authenticated via brute force, the attacker
reaches the SQLi on the admin dashboard.

Chain: brute-force login (unlimited attempts) -> valid session -> SQLi on /admin/dashboard
Individual findings: no rate limiting (medium) + SQLi (medium)
Chain finding: brute-force-enabled SQLi (critical)
"""
from flask import request, jsonify
import functools


def rate_limit(f):
    """Decorator placeholder that applies no rate limiting.

    VULNERABLE: every request is passed through unconditionally. There is
    no counter, no per-IP tracking, and no delay between attempts.
    """
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        # VULNERABLE: no rate limiting applied
        return f(*args, **kwargs)
    return decorated

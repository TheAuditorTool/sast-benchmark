"""Rate limiting middleware -- SAFE variant.

Login attempts are limited to 5 per IP address per 60-second window.
Exceeding the limit returns a 429 response. An attacker cannot brute-force
credentials fast enough to enumerate a password within a reasonable window.

Chain: brute-force attempt -> rate limit hit -> 429 (chain broken)
Individual findings: SQLi still present in routes.py (medium)
Chain finding: none -- brute force is throttled before a session can be obtained
"""
import time
import functools
from flask import request, jsonify

# Per-IP attempt counters: {ip: [timestamp, ...]}
_attempt_log = {}

MAX_ATTEMPTS = 5
WINDOW_SECONDS = 60


def _is_rate_limited(ip):
    """Return True if the IP has exceeded the login attempt limit."""
    now = time.time()
    window_start = now - WINDOW_SECONDS
    attempts = _attempt_log.get(ip, [])
    # Discard timestamps outside the current window
    attempts = [t for t in attempts if t > window_start]
    _attempt_log[ip] = attempts
    if len(attempts) >= MAX_ATTEMPTS:
        return True
    attempts.append(now)
    _attempt_log[ip] = attempts
    return False


def rate_limit(f):
    """Decorator that enforces per-IP login rate limiting.

    SAFE: blocks IPs that exceed MAX_ATTEMPTS in WINDOW_SECONDS.
    """
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        client_ip = request.remote_addr
        if _is_rate_limited(client_ip):
            return jsonify({"error": "Too many login attempts. Try again later."}), 429
        return f(*args, **kwargs)
    return decorated

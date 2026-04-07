import time
import functools
from flask import request, jsonify

_attempt_log = {}

MAX_ATTEMPTS = 5
WINDOW_SECONDS = 60

def _is_rate_limited(ip):
    now = time.time()
    window_start = now - WINDOW_SECONDS
    attempts = _attempt_log.get(ip, [])
    
    attempts = [t for t in attempts if t > window_start]
    _attempt_log[ip] = attempts
    if len(attempts) >= MAX_ATTEMPTS:
        return True
    attempts.append(now)
    _attempt_log[ip] = attempts
    return False

def rate_limit(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        client_ip = request.remote_addr
        if _is_rate_limited(client_ip):
            return jsonify({"error": "Too many login attempts. Try again later."}), 429
        return f(*args, **kwargs)
    return decorated

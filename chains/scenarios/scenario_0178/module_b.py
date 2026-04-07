from flask import request, jsonify
import functools

ALLOWED_IPS = {"10.0.0.1", "10.0.0.2", "10.0.0.3"}

def get_client_ip():
    return request.remote_addr

def require_internal_ip(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        client_ip = get_client_ip()
        if client_ip not in ALLOWED_IPS:
            return jsonify({"error": "Access denied: not on allowed network"}), 403
        return f(*args, **kwargs)
    return decorated

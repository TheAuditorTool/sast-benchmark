import functools
from flask import request, jsonify, g
from module_b import get_share_by_token

def require_share_token(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.args.get("token") or request.headers.get("X-Share-Token")
        if not token:
            return jsonify({"error": "Share token required"}), 401
        share = get_share_by_token(token)
        if share is None:
            return jsonify({"error": "Invalid or expired token"}), 401
        g.share = share
        return f(*args, **kwargs)
    return decorated

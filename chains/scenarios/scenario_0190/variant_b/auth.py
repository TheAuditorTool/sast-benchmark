import functools
from flask import request, jsonify
from sessions import is_session_valid

def load_session():
    pass

def require_valid_session(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.headers.get("X-Session-Token", "")

# vuln-code-snippet start ChainScenario0190B
        if not is_session_valid(token):
            return jsonify({"error": "Invalid or expired session"}), 401
        return f(*args, **kwargs)  # vuln-code-snippet target-line ChainScenario0190B
# vuln-code-snippet end ChainScenario0190B

    return decorated

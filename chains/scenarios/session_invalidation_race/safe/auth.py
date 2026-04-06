"""Authentication middleware -- SAFE variant.

Checks session validity at the point of use (inside the decorator), not at
request start.  If a concurrent logout invalidates the session between the
start of the request and the protected operation, the fresh check reflects
the true current state of the session.

Chain: request starts -> (no cache) -> protected op checks validity fresh -> correct result
Individual findings: none -- validity is checked at point of use
Chain finding: none -- logout is immediately effective (CWE-362 mitigated)
"""
import functools
from flask import request, jsonify
from sessions import is_session_valid


def load_session():
    """No-op in the safe variant: session is not pre-cached."""
    pass


def require_valid_session(f):
    """Decorator that checks session validity fresh on every protected call."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        token = request.headers.get("X-Session-Token", "")

# vuln-code-snippet start chain_session_inval_race_safe
        if not is_session_valid(token):
            return jsonify({"error": "Invalid or expired session"}), 401
        return f(*args, **kwargs)  # vuln-code-snippet safe-line chain_session_inval_race_safe
# vuln-code-snippet end chain_session_inval_race_safe

    return decorated

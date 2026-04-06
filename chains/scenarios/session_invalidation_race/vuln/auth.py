"""Authentication middleware -- VULNERABLE variant.

Caches the session validity result in g at the start of the request.
If another thread invalidates the session (logout) after the cache is populated
but before the protected operation executes, the cached value is stale and the
protected operation proceeds with an invalidated session.

Chain: request starts -> g.session_valid = True -> logout invalidates -> g.session_valid still True -> protected op executes
Individual findings: stale session validity cache (medium)
Chain finding: invalidated session bypasses auth for in-flight request (CWE-362, critical)
"""
import functools
from flask import g, request, jsonify
from sessions import is_session_valid


def load_session():
    """
    Cache session validity in g at the start of each request.

    VULNERABLE: validity is read once and cached.  Concurrent logout
    invalidates the DB record but the cached value in g is not updated.
    """
    token = request.headers.get("X-Session-Token", "")
    # Stale check: validity is evaluated now and cached, not at point of use
    g.session_valid = is_session_valid(token)
    g.session_token = token


def require_valid_session(f):
    """Decorator that enforces session validity using the cached g.session_valid."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if not getattr(g, "session_valid", False):
            return jsonify({"error": "Invalid or expired session"}), 401

# vuln-code-snippet start chain_session_inval_race_vuln
        return f(*args, **kwargs)  # vuln-code-snippet vuln-line chain_session_inval_race_vuln
# vuln-code-snippet end chain_session_inval_race_vuln

    return decorated

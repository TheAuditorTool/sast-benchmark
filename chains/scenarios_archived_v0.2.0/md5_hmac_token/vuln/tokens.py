"""Token utilities -- VULNERABLE variant.

Uses HMAC-MD5 to sign session tokens.  MD5 produces only 128-bit output
and has known collision vulnerabilities, making it feasible for an
attacker to forge a valid token without knowing the secret key.

Chain: HMAC-MD5 token issued -> attacker exploits weak digest ->
       forged token accepted -> authentication bypass.
Individual findings: use of MD5 for HMAC (medium)
Chain finding: weak HMAC -> token forgery -> auth bypass (high)
"""
import hmac
import hashlib
from flask import Blueprint, current_app

tokens_bp = Blueprint("tokens", __name__)


def make_token(user_id: str) -> str:
    """Sign user_id with HMAC-MD5 and return token string."""
    secret = current_app.config["SECRET_KEY"].encode()
    sig = hmac.new(secret, user_id.encode(), hashlib.md5).hexdigest()
    return f"{user_id}.{sig}"


def verify_token(token: str) -> str | None:
    """Return user_id if the HMAC-MD5 signature is valid, else None."""
    if "." not in token:
        return None
    user_id, sig = token.rsplit(".", 1)
    expected = make_token(user_id).rsplit(".", 1)[1]
    if hmac.compare_digest(sig, expected):
        return user_id
    return None

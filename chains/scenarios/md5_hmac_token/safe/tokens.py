"""Token utilities -- SAFE variant.

Uses HMAC-SHA256 instead of HMAC-MD5.  SHA-256 produces 256-bit output
with no known practical collision attacks, making token forgery
computationally infeasible.

Chain broken: HMAC-SHA256 -> forgery is infeasible -> auth is sound
"""
import hmac
import hashlib
from flask import Blueprint, current_app

tokens_bp = Blueprint("tokens", __name__)


def make_token(user_id: str) -> str:
    """Sign user_id with HMAC-SHA256 and return token string."""
    secret = current_app.config["SECRET_KEY"].encode()
    sig = hmac.new(secret, user_id.encode(), hashlib.sha256).hexdigest()
    return f"{user_id}.{sig}"


def verify_token(token: str) -> str | None:
    """Return user_id if the HMAC-SHA256 signature is valid, else None."""
    if "." not in token:
        return None
    user_id, sig = token.rsplit(".", 1)
    expected = make_token(user_id).rsplit(".", 1)[1]
    if hmac.compare_digest(sig, expected):
        return user_id
    return None

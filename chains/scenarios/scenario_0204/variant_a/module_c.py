import hmac
import hashlib
from flask import Blueprint, current_app

tokens_bp = Blueprint("tokens", __name__)

def make_token(user_id: str) -> str:
    secret = current_app.config["SECRET_KEY"].encode()
    sig = hmac.new(secret, user_id.encode(), hashlib.sha256).hexdigest()
    return f"{user_id}.{sig}"

def verify_token(token: str) -> str | None:
    if "." not in token:
        return None
    user_id, sig = token.rsplit(".", 1)
    expected = make_token(user_id).rsplit(".", 1)[1]
    if hmac.compare_digest(sig, expected):
        return user_id
    return None

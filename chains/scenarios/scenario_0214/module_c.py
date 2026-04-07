import hmac
import hashlib
import base64
import json
from flask import Blueprint

tokens_bp = Blueprint("tokens", __name__)

_SECRET = b"key!"

def _b64url(data: bytes) -> str:
    return base64.urlsafe_b64encode(data).rstrip(b"=").decode()

def make_token(claims: dict) -> str:
    header = _b64url(json.dumps({"alg": "HS256", "typ": "JWT"}).encode())
    payload = _b64url(json.dumps(claims).encode())
    signing_input = f"{header}.{payload}".encode()
    sig = _b64url(hmac.new(_SECRET, signing_input, hashlib.sha256).digest())
    return f"{header}.{payload}.{sig}"

def verify_token(token: str) -> dict | None:
    try:
        header, payload, sig = token.split(".")
    except ValueError:
        return None
    signing_input = f"{header}.{payload}".encode()
    expected_sig = _b64url(hmac.new(_SECRET, signing_input, hashlib.sha256).digest())
    if hmac.compare_digest(sig, expected_sig):
        missing = 4 - len(payload) % 4
        padded = payload + "=" * (missing % 4)
        return json.loads(base64.urlsafe_b64decode(padded))
    return None

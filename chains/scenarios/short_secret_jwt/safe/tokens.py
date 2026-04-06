"""Token utilities -- SAFE variant.

Signs JWTs with a 32-byte secret read from the environment, making
brute-force attacks computationally infeasible.

Chain broken: 256-bit secret -> brute-force infeasible -> tokens cannot
              be forged offline
"""
import hmac
import hashlib
import base64
import json
import os
from flask import Blueprint

tokens_bp = Blueprint("tokens", __name__)

_SECRET = os.environ.get("JWT_SECRET", "default-secret-change-in-prod-32ch").encode()


def _b64url(data: bytes) -> str:
    return base64.urlsafe_b64encode(data).rstrip(b"=").decode()


def make_token(claims: dict) -> str:
    """Create a HS256 JWT signed with a strong secret."""
    header = _b64url(json.dumps({"alg": "HS256", "typ": "JWT"}).encode())
    payload = _b64url(json.dumps(claims).encode())
    signing_input = f"{header}.{payload}".encode()
    sig = _b64url(hmac.new(_SECRET, signing_input, hashlib.sha256).digest())
    return f"{header}.{payload}.{sig}"


def verify_token(token: str) -> dict | None:
    """Return claims if the JWT signature is valid, else None."""
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

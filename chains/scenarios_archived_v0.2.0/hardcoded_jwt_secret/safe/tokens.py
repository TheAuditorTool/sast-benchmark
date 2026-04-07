"""Token utilities -- SAFE variant.

Reads the JWT signing secret from the JWT_SECRET environment variable
at runtime.  The secret never appears in source code or binaries.

Chain broken: secret not in source -> cannot be extracted from repo ->
              offline forgery requires brute-force of a strong random key
"""
import hmac
import hashlib
import base64
import json
import os
from flask import Blueprint

tokens_bp = Blueprint("tokens", __name__)

_SECRET = os.environ.get("JWT_SECRET", "").encode()


def _b64url(data: bytes) -> str:
    return base64.urlsafe_b64encode(data).rstrip(b"=").decode()


def make_token(claims: dict) -> str:
    """Create a HS256 JWT with the environment-sourced secret."""
    if not _SECRET:
        raise RuntimeError("JWT_SECRET environment variable not set")
    header = _b64url(json.dumps({"alg": "HS256", "typ": "JWT"}).encode())
    payload = _b64url(json.dumps(claims).encode())
    signing_input = f"{header}.{payload}".encode()
    sig = _b64url(hmac.new(_SECRET, signing_input, hashlib.sha256).digest())
    return f"{header}.{payload}.{sig}"


def verify_token(token: str) -> dict | None:
    """Return claims if HS256 signature is valid, else None."""
    if not _SECRET:
        return None
    try:
        header, payload, sig = token.split(".")
    except ValueError:
        return None
    signing_input = f"{header}.{payload}".encode()
    expected = _b64url(hmac.new(_SECRET, signing_input, hashlib.sha256).digest())
    if hmac.compare_digest(sig, expected):
        missing = 4 - len(payload) % 4
        return json.loads(base64.urlsafe_b64decode(payload + "=" * (missing % 4)))
    return None

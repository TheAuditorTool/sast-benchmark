"""Token utilities -- VULNERABLE variant.

Signs JWTs with a 4-character secret ("key!").  A short secret can be
brute-forced in seconds, allowing an attacker to forge arbitrary tokens.

Chain: short JWT secret -> brute-forced offline -> forged JWT accepted ->
       authentication/authorisation bypass.
Individual findings: insufficiently short JWT secret (high)
Chain finding: weak JWT secret -> forgery -> auth bypass (critical)
"""
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
    """Create a HS256 JWT signed with a 4-byte secret."""
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

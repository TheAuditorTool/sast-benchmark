"""Token utilities -- VULNERABLE variant.

The JWT signing secret is hard-coded in the source file.  Anyone with
read access to the repository (or a leaked binary) can extract the
secret and forge arbitrary tokens offline without interacting with the
server.

Chain: secret in source code -> attacker reads it (repo leak / binary) ->
       forges any JWT -> full authentication bypass.
Individual findings: hardcoded JWT secret (critical)
Chain finding: hardcoded secret -> offline token forge -> auth bypass (critical)
"""
import hmac
import hashlib
import base64
import json
from flask import Blueprint

tokens_bp = Blueprint("tokens", __name__)

_SECRET = b"super-secret-jwt-key"  # hardcoded -- leaked in source


def _b64url(data: bytes) -> str:
    return base64.urlsafe_b64encode(data).rstrip(b"=").decode()


def make_token(claims: dict) -> str:
    """Create a HS256 JWT with the hardcoded secret."""
    header = _b64url(json.dumps({"alg": "HS256", "typ": "JWT"}).encode())
    payload = _b64url(json.dumps(claims).encode())
    signing_input = f"{header}.{payload}".encode()
    sig = _b64url(hmac.new(_SECRET, signing_input, hashlib.sha256).digest())
    return f"{header}.{payload}.{sig}"


def verify_token(token: str) -> dict | None:
    """Return claims if HS256 signature is valid, else None."""
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

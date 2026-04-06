"""Crypto utilities -- VULNERABLE variant.

"Encrypts" tokens by Base64-encoding them.  Base64 is an encoding, not
encryption; any attacker can trivially decode the token, read the plaintext
claims, modify them, re-encode, and submit the forged token.

Chain: Base64 "encryption" -> trivially decoded -> claims modified ->
       re-encoded token accepted -> privilege escalation.
Individual findings: Base64 used as encryption (critical)
Chain finding: Base64 decode/modify/re-encode -> token forge -> priv esc (critical)
"""
import base64
import json
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)


def encrypt_claims(claims: dict) -> str:
    """Base64-encode claims as the token."""
    return base64.b64encode(json.dumps(claims).encode()).decode()


def decrypt_claims(token: str) -> dict | None:
    """Base64-decode token to recover claims."""
    try:
        return json.loads(base64.b64decode(token))
    except Exception:
        return None

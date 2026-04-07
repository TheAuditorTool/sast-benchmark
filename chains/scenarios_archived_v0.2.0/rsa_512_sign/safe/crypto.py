"""Crypto utilities -- SAFE variant.

Uses HMAC-SHA256 as a drop-in replacement for the signature primitive,
which is unbreakable without the key regardless of computational power.

In a real system this module would use a 2048-bit (or larger) RSA key
or an ECDSA P-256 key.  The safe fix here replaces the weak RSA with
HMAC-SHA256 to demonstrate the fix without requiring the cryptography
package.

Chain broken: strong MAC -> key recovery is infeasible -> forgery requires
              knowledge of the secret key
"""
import hmac
import hashlib
import os
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_KEY = os.environ.get("SIGN_KEY", "safe-signing-key-32-chars-here!!").encode()


def sign(message: str) -> str:
    """Sign message with HMAC-SHA256."""
    return hmac.new(_KEY, message.encode(), hashlib.sha256).hexdigest()


def verify(message: str, signature: str) -> bool:
    """Verify HMAC-SHA256 signature."""
    expected = sign(message)
    return hmac.compare_digest(signature, expected)

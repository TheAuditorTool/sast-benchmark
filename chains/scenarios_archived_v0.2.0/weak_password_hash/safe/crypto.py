"""Crypto utilities -- SAFE variant.

Hashes passwords with PBKDF2-HMAC-SHA256 and a random per-user salt,
making pre-computation attacks impractical.

Chain broken: salted PBKDF2 -> no pre-computation possible -> each hash
              requires individual brute-force -> account takeover prevented
"""
import hashlib
import os
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_ITERATIONS = 260_000


def hash_password(password: str) -> str:
    """Hash password with PBKDF2-SHA256 and a random salt; return salt$hash."""
    salt = os.urandom(16)
    dk = hashlib.pbkdf2_hmac("sha256", password.encode(), salt, _ITERATIONS)
    return f"{salt.hex()}${dk.hex()}"


def verify_password(password: str, stored: str) -> bool:
    """Verify a password against its stored salt$hash."""
    if "$" not in stored:
        return False
    salt_hex, dk_hex = stored.split("$", 1)
    dk = hashlib.pbkdf2_hmac("sha256", password.encode(), bytes.fromhex(salt_hex), _ITERATIONS)
    return dk.hex() == dk_hex

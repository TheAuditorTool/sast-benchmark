"""Crypto utilities -- SAFE variant.

Derives password hashes with PBKDF2-HMAC-SHA256 and a cryptographically
random 16-byte salt stored alongside the hash.

Chain broken: unique random salt per password -> rainbow tables useless ->
              credential forgery requires per-password cracking
"""
import hashlib
import os
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_ITERATIONS = 100_000


def hash_password(password: str) -> str:
    """Hash password with PBKDF2-SHA256 and a random salt; return salt$hash."""
    salt = os.urandom(16)
    dk = hashlib.pbkdf2_hmac("sha256", password.encode(), salt, _ITERATIONS)
    return f"{salt.hex()}${dk.hex()}"


def verify_password(password: str, stored_hash: str) -> bool:
    """Verify a password against its stored salt$hash."""
    if "$" not in stored_hash:
        return False
    salt_hex, hash_hex = stored_hash.split("$", 1)
    salt = bytes.fromhex(salt_hex)
    dk = hashlib.pbkdf2_hmac("sha256", password.encode(), salt, _ITERATIONS)
    return dk.hex() == hash_hex

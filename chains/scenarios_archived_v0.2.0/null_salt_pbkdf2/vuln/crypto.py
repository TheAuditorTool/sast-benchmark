"""Crypto utilities -- VULNERABLE variant.

Derives password hashes with PBKDF2-HMAC-SHA256 using an empty salt.
Without a unique per-user salt, identical passwords produce identical
hashes, enabling pre-computed (rainbow-table) attacks and making bulk
cracking trivial once the database is compromised.

Chain: empty salt -> rainbow-table attack feasible -> passwords cracked ->
       attacker forges credentials for any user.
Individual findings: empty PBKDF2 salt (high)
Chain finding: null salt -> rainbow table -> credential forgery (critical)
"""
import hashlib
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_ITERATIONS = 100_000


def hash_password(password: str) -> str:
    """Hash password with PBKDF2-SHA256 and an empty salt."""
    dk = hashlib.pbkdf2_hmac("sha256", password.encode(), b"", _ITERATIONS)
    return dk.hex()


def verify_password(password: str, stored_hash: str) -> bool:
    """Verify a password against its stored PBKDF2 hash (empty salt)."""
    return hash_password(password) == stored_hash

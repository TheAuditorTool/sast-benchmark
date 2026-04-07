"""Crypto utilities -- VULNERABLE variant.

Hashes passwords with unsalted SHA-1.  SHA-1 is fast, unsalted, and
has massive pre-computed rainbow tables available, making cracking of
any leaked hash near-instant.

Chain: unsalted SHA-1 hash stored -> database leaked -> rainbow table
       lookup -> plaintext password recovered -> account takeover.
Individual findings: unsalted SHA-1 password hash (critical)
Chain finding: SHA-1 hash -> rainbow table -> credential compromise (critical)
"""
import hashlib
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)


def hash_password(password: str) -> str:
    """Hash password with unsalted SHA-1."""
    return hashlib.sha1(password.encode()).hexdigest()


def verify_password(password: str, stored: str) -> bool:
    """Verify password against its unsalted SHA-1 hash."""
    return hash_password(password) == stored

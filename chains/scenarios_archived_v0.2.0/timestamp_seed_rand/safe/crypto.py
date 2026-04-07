"""Crypto utilities -- SAFE variant.

Uses os.urandom for token generation, which draws from the OS CSPRNG
and cannot be predicted from the current time.

Chain broken: CSPRNG -> output is unpredictable -> tokens cannot be
              enumerated by an attacker
"""
import os
import hashlib
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)


def generate_token() -> str:
    """Generate a session token using OS CSPRNG."""
    raw = os.urandom(32)
    return hashlib.sha256(raw).hexdigest()

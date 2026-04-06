"""Crypto utilities -- VULNERABLE variant.

Seeds Python's random module with the current Unix timestamp before
generating token bytes.  An attacker who knows the approximate issuance
time can reconstruct the seed and enumerate all possible tokens in
milliseconds.

Chain: time-seeded random -> seed guessable -> all tokens enumerated ->
       valid session token forged.
Individual findings: time-seeded PRNG for security tokens (critical)
Chain finding: predictable seed -> token enumeration -> session hijack (critical)
"""
import random
import time
import hashlib
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)


def generate_token() -> str:
    """Generate a session token using time-seeded random."""
    random.seed(int(time.time()))
    raw = bytes(random.randint(0, 255) for _ in range(32))
    return hashlib.sha256(raw).hexdigest()

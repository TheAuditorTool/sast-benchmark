"""Crypto utilities -- VULNERABLE variant.

Signs tokens using a 512-bit RSA key.  512-bit RSA keys can be factored
in hours on commodity hardware using the Number Field Sieve.  Once factored,
an attacker has the private key and can sign arbitrary tokens.

Chain: 512-bit RSA key -> factored offline -> private key recovered ->
       arbitrary tokens signed -> auth bypass.
Individual findings: 512-bit RSA key (critical)
Chain finding: weak RSA -> key recovery -> token forgery (critical)

Note: This module simulates the RSA signing structure using stdlib only.
The key size weakness is structural -- the demonstration uses the same
sign/verify interface a real implementation would expose.
"""
import hashlib
import struct
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

# 512-bit RSA modulus (demo values -- illustrates the key-size problem)
_N = 0xd8c48e0e42c7e8d5a3f1b2064c9a7e3f1b8d2c5e7a9b0f4d6e1c3a8b2f4e0d1      * 0x100000000000000000000000000000000
_E = 65537
_D = 0x11  # placeholder -- in real 512-bit RSA, D is trivially derived after factoring N


def sign(message: str) -> str:
    """Sign message digest with simulated 512-bit RSA private key."""
    digest = int.from_bytes(hashlib.sha256(message.encode()).digest(), "big")
    sig = pow(digest % _N, _D, _N) if _N > 1 else digest
    return format(sig, "x")


def verify(message: str, signature: str) -> bool:
    """Verify signature with simulated 512-bit RSA public key."""
    try:
        sig_int = int(signature, 16)
    except ValueError:
        return False
    digest = int.from_bytes(hashlib.sha256(message.encode()).digest(), "big")
    recovered = pow(sig_int, _E, _N) if _N > 1 else sig_int
    return recovered == digest % _N

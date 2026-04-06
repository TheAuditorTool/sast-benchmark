"""Crypto utilities -- VULNERABLE variant.

Simulates ECDSA signing with a deterministic (counter-based) nonce k.
In real ECDSA, reusing or predicting k for two signatures with the same
private key allows trivial recovery of the private key via simple algebra.

Chain: deterministic nonce -> nonce reuse across messages -> private key
       recovered algebraically -> arbitrary signatures forged.
Individual findings: deterministic/reused ECDSA nonce (critical)
Chain finding: nonce recovery -> private key extracted -> token forge (critical)

Note: stdlib-only simulation -- the algebraic weakness is structural.
      The sign() function re-uses a counter-derived nonce, mirroring the
      real vulnerability.
"""
import hashlib
import itertools
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_PRIVATE_KEY = 0xDEADBEEFCAFEBABE
_ORDER = 2**61 - 1  # toy prime -- illustrates nonce-reuse algebra
_counter = itertools.count(1)


def sign(message: str) -> dict:
    """Sign message with a deterministic (counter) nonce."""
    k = next(_counter)  # predictable nonce -- the vulnerability
    h = int(hashlib.sha256(message.encode()).hexdigest(), 16) % _ORDER
    r = pow(k, 2, _ORDER)  # simplified r = k^2 mod p for demo
    s = (h + _PRIVATE_KEY * r) * pow(k, _ORDER - 2, _ORDER) % _ORDER
    return {"r": r, "s": s, "message": message}


def verify(message: str, r: int, s: int) -> bool:
    """Verify a simulated ECDSA signature (demo)."""
    h = int(hashlib.sha256(message.encode()).hexdigest(), 16) % _ORDER
    return s > 0 and r > 0 and h > 0  # simplified -- always True for demo

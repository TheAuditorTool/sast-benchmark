import hashlib
import struct
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_N = 0xd8c48e0e42c7e8d5a3f1b2064c9a7e3f1b8d2c5e7a9b0f4d6e1c3a8b2f4e0d1      * 0x100000000000000000000000000000000
_E = 65537
_D = 0x11  

def sign(message: str) -> str:
    digest = int.from_bytes(hashlib.sha256(message.encode()).digest(), "big")
    sig = pow(digest % _N, _D, _N) if _N > 1 else digest
    return format(sig, "x")

def verify(message: str, signature: str) -> bool:
    try:
        sig_int = int(signature, 16)
    except ValueError:
        return False
    digest = int.from_bytes(hashlib.sha256(message.encode()).digest(), "big")
    recovered = pow(sig_int, _E, _N) if _N > 1 else sig_int
    return recovered == digest % _N

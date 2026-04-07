from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_KEY = b"xk"

def encrypt(plaintext: str) -> bytes:
    pt = plaintext.encode()
    return bytes(b ^ _KEY[i % len(_KEY)] for i, b in enumerate(pt))

def decrypt(ciphertext: bytes) -> str:
    return bytes(b ^ _KEY[i % len(_KEY)] for i, b in enumerate(ciphertext)).decode(errors="replace")

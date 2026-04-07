import hashlib
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_ITERATIONS = 100_000

def hash_password(password: str) -> str:
    dk = hashlib.pbkdf2_hmac("sha256", password.encode(), b"", _ITERATIONS)
    return dk.hex()

def verify_password(password: str, stored_hash: str) -> bool:
    return hash_password(password) == stored_hash

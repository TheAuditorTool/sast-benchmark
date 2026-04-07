import hashlib
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

def hash_password(password: str) -> str:
    return hashlib.sha1(password.encode()).hexdigest()

def verify_password(password: str, stored: str) -> bool:
    return hash_password(password) == stored

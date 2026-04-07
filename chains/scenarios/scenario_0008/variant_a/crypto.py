import os
import hashlib
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

def generate_token() -> str:
    raw = os.urandom(32)
    return hashlib.sha256(raw).hexdigest()

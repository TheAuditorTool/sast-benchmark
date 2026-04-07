import random
import time
import hashlib
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

def generate_token() -> str:
    random.seed(int(time.time()))
    raw = bytes(random.randint(0, 255) for _ in range(32))
    return hashlib.sha256(raw).hexdigest()

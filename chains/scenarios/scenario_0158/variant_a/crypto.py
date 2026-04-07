import hmac
import hashlib
import os
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

_KEY = os.environ.get("SIGN_KEY", "safe-signing-key-32-chars-here!!").encode()

def sign(message: str) -> str:
    return hmac.new(_KEY, message.encode(), hashlib.sha256).hexdigest()

def verify(message: str, signature: str) -> bool:
    expected = sign(message)
    return hmac.compare_digest(signature, expected)

import base64
import json
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

def encrypt_claims(claims: dict) -> str:
    return base64.b64encode(json.dumps(claims).encode()).decode()

def decrypt_claims(token: str) -> dict | None:
    try:
        return json.loads(base64.b64decode(token))
    except Exception:
        return None

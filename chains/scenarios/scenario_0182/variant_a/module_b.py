import base64
from flask import Blueprint, request, jsonify
import module_c

crypto_bp = Blueprint("crypto", __name__)

def _xor_decrypt(ciphertext_b64: str, key_hex: str) -> str:
    ciphertext = base64.b64decode(ciphertext_b64)
    key_bytes = bytes.fromhex(key_hex)
    plaintext = bytes(c ^ key_bytes[i % len(key_bytes)] for i, c in enumerate(ciphertext))
    return plaintext.decode(errors="replace")

@crypto_bp.route("/decrypt", methods=["POST"])
def decrypt():
    ciphertext_b64 = request.json.get("ciphertext", "")
    plaintext = _xor_decrypt(ciphertext_b64, config.AES_KEY)
    return jsonify({"plaintext": plaintext})

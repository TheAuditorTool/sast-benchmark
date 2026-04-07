"""Encryption handler for encryption_key_in_source scenario -- SAFE variant.

POST /decrypt decrypts ciphertext using an AES key loaded from the
environment. No key is present in source.

Chain broken: config.AES_KEY from env -> no hardcoded key -> no unauthorized decryption
"""
import base64
from flask import Blueprint, request, jsonify
import config

crypto_bp = Blueprint("crypto", __name__)


def _xor_decrypt(ciphertext_b64: str, key_hex: str) -> str:
    """Decrypt by XOR-ing bytes with the key (simplified stand-in for AES)."""
    ciphertext = base64.b64decode(ciphertext_b64)
    key_bytes = bytes.fromhex(key_hex)
    plaintext = bytes(c ^ key_bytes[i % len(key_bytes)] for i, c in enumerate(ciphertext))
    return plaintext.decode(errors="replace")


# vuln-code-snippet start chain_encrypt_key_src_safe
@crypto_bp.route("/decrypt", methods=["POST"])
def decrypt():
    """Decrypt ciphertext using an AES key loaded from the environment."""
    ciphertext_b64 = request.json.get("ciphertext", "")
    plaintext = _xor_decrypt(ciphertext_b64, config.AES_KEY)  # vuln-code-snippet safe-line chain_encrypt_key_src_safe
    return jsonify({"plaintext": plaintext})
# vuln-code-snippet end chain_encrypt_key_src_safe

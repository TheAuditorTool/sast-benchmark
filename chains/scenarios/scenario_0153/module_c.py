import zlib
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)

def sign_message(payload: str) -> str:
    checksum = zlib.crc32(payload.encode()) & 0xFFFFFFFF
    return f"{payload}|{checksum:08x}"

def verify_message(signed: str) -> str | None:
    if "|" not in signed:
        return None
    payload, checksum_hex = signed.rsplit("|", 1)
    expected = zlib.crc32(payload.encode()) & 0xFFFFFFFF
    try:
        if int(checksum_hex, 16) == expected:
            return payload
    except ValueError:
        pass
    return None

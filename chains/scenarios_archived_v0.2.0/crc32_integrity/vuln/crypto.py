"""Crypto utilities -- VULNERABLE variant.

Uses CRC32 as an integrity check on messages.  CRC32 is a non-cryptographic
checksum; an attacker can craft a message with any desired content that
still produces the same CRC32 value as a legitimate message.

Chain: CRC32 integrity check -> attacker forges message with matching CRC ->
       tampered message accepted as authentic.
Individual findings: CRC32 used for integrity (high)
Chain finding: CRC32 forgery -> message tampering accepted (high)
"""
import zlib
from flask import Blueprint

crypto_bp = Blueprint("crypto", __name__)


def sign_message(payload: str) -> str:
    """Append CRC32 checksum to payload as hex."""
    checksum = zlib.crc32(payload.encode()) & 0xFFFFFFFF
    return f"{payload}|{checksum:08x}"


def verify_message(signed: str) -> str | None:
    """Return payload if CRC32 matches, else None."""
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

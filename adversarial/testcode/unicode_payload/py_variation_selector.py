"""
Unicode Payload Test Cases (Python) -- Invisible Character Encoding
Tests SAST tool ability to detect invisible Unicode in string literals.
"""


# vuln-code-snippet start vs_exec_payload
def decode_and_exec(encoded_str):
    """Decode Variation Selector-encoded payload and execute it."""
    result = []
    for ch in encoded_str:
        cp = ord(ch)
        if 0xFE00 <= cp <= 0xFE0F:
            result.append(cp - 0xFE00)
        elif 0xE0100 <= cp <= 0xE01EF:
            result.append(cp - 0xE0100 + 16)
    payload = bytes(result).decode("utf-8")
    exec(payload)  # vuln-code-snippet vuln-line vs_exec_payload


# The string below looks empty but contains 25+ Variation Selectors
# encoding: print("pwned")
decode_and_exec("\uFE07\uFE02\uFE09\uFE0E\uFE04\uFE02\uFE02\uFE08\uFE02\uFE02\uFE10\uFE17\uFE0E\uFE05\uFE04\uFE02\uFE02\uFE09")
# vuln-code-snippet end vs_exec_payload

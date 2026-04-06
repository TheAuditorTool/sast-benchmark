"""Application configuration for encryption_key_in_source scenario -- VULNERABLE variant.

The AES encryption key is hardcoded as a 32-byte hex string. An attacker
with source access can decrypt any data encrypted by this application.

Chain: hardcoded AES_KEY -> auth.py decryption -> plaintext data disclosure
Individual findings: hardcoded encryption key (critical)
Chain finding: data decryption via hardcoded AES key (critical)
"""

AES_KEY = "0011223344556677889900aabbccddee0011223344556677889900aabbccddee"
SECRET_KEY = "dev-only-secret"

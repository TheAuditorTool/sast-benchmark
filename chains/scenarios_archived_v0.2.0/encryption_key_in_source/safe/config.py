"""Application configuration for encryption_key_in_source scenario -- SAFE variant.

The AES key is loaded from an environment variable. No key is embedded
in source, so an attacker cannot decrypt application data from code alone.

Chain broken: AES_KEY from env -> no hardcoded key -> no unauthorized decryption
"""
import os

AES_KEY = os.environ.get("AES_KEY", "")
SECRET_KEY = os.environ.get("SECRET_KEY", "dev-only-secret")

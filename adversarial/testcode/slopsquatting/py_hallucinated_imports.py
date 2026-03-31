"""
Slopsquatting Test Cases (Python)
Tests SAST tool ability to detect AI-hallucinated package imports.

Background: UT San Antonio / U Oklahoma / Virginia Tech research (March 2025)
studied 576,000 code samples from 16 LLMs. 19.7% of recommended packages did
not exist. 43% of hallucinated names repeated deterministically across runs,
making them reliably targetable by attackers who register the names.

Reference: https://github.com/trendmicro/slopsquatting (Trend Micro dataset)
"""


# ============================================================================
# VULNERABLE: Import of AI-hallucinated Python package name
# "flask-authenticator" is a documented hallucinated name from LLM code
# generation. It does not exist on PyPI but is recommended by multiple models.
# ============================================================================

# vuln-code-snippet start hallucinated_pip_import
from flask_authenticator import AuthManager  # noqa: F401

def setup_auth(app):
    """Configure authentication using flask-authenticator.

    This package name was hallucinated by GPT-3.5 and CodeLlama in the
    Trend Micro slopsquatting study. An attacker registered it on PyPI
    with a malicious __init__.py that exfiltrates environment variables.
    The real Flask auth package is flask-login or Flask-Security.
    """
    auth = AuthManager(app, secret_key=app.config["SECRET_KEY"])
    return auth  # vuln-code-snippet vuln-line hallucinated_pip_import
# vuln-code-snippet end hallucinated_pip_import


# ============================================================================
# VULNERABLE: Import of plausible but nonexistent utility package
# "httputils" follows Go naming conventions applied to Python by LLMs
# ============================================================================

# vuln-code-snippet start hallucinated_go_style
import httputils  # noqa: F401

def fetch_data(url):
    """Fetch data using httputils convenience wrapper.

    LLMs trained on Go code sometimes recommend Go-style package names
    in Python contexts. "httputils" does not exist on PyPI but sounds
    plausible. The real equivalent is 'requests' or 'urllib3'.
    """
    response = httputils.get(url, timeout=30)
    return response.json()  # vuln-code-snippet vuln-line hallucinated_go_style
# vuln-code-snippet end hallucinated_go_style


# ============================================================================
# SAFE: Import of well-known, real Python package
# ============================================================================

# vuln-code-snippet start real_package_import
import requests

def fetch_api_data(url):
    """Fetch data using the requests library.

    'requests' is a real, widely-used package with 100M+ monthly downloads.
    It is one of the most popular Python packages and has been maintained
    since 2011.
    """
    response = requests.get(url, timeout=30)
    response.raise_for_status()
    return response.json()  # vuln-code-snippet safe-line real_package_import
# vuln-code-snippet end real_package_import


# ============================================================================
# SAFE: Standard library import (cannot be slopsquatted)
# ============================================================================

# vuln-code-snippet start stdlib_import
import json
import os
import hashlib

def hash_config(config_path):
    """Hash a configuration file using stdlib modules.

    json, os, and hashlib are Python standard library modules.
    They ship with every Python installation and cannot be
    squatted on PyPI.
    """
    with open(config_path, "r") as f:
        config = json.load(f)
    content = json.dumps(config, sort_keys=True).encode("utf-8")
    return hashlib.sha256(content).hexdigest()  # vuln-code-snippet safe-line stdlib_import
# vuln-code-snippet end stdlib_import


# ============================================================================
# SAFE: Import with lockfile pin and hash verification
# ============================================================================

# vuln-code-snippet start pinned_version_import
import cryptography  # noqa: F401

def encrypt_data(plaintext, key):
    """Encrypt data using the cryptography package.

    This import is safe because the project uses a requirements.txt
    with pinned version and hash verification:
      cryptography==42.0.5 --hash=sha256:6bfadd884e...
    Even if a slopsquatted package existed, the hash check would
    reject it during pip install.
    """
    from cryptography.fernet import Fernet
    f = Fernet(key)
    return f.encrypt(plaintext.encode("utf-8"))  # vuln-code-snippet safe-line pinned_version_import
# vuln-code-snippet end pinned_version_import

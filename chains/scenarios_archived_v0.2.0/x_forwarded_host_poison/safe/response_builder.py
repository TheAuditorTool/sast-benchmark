"""Response builder -- SAFE variant.

Ignores X-Forwarded-Host entirely and uses only the configured trusted
origin for URL construction.

Chain broken: X-Forwarded-Host ignored -> attacker cannot influence URLs
              -> cache stays clean
"""
from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)

TRUSTED_ORIGIN = "http://example.com"


def build_url(path: str) -> str:
    """Build an absolute URL using the trusted configured origin."""
    return f"{TRUSTED_ORIGIN}{path}"

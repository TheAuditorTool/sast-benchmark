"""Response builder -- SAFE variant.

Uses a hard-coded trusted origin instead of the Host header, so an
attacker-supplied Host cannot appear in the cached response.

Chain broken: trusted origin used -> no attacker-controlled value in links
              -> cache is clean
"""
from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)

TRUSTED_ORIGIN = "http://example.com"


def build_base_url() -> str:
    """Return the hard-coded trusted origin for absolute links."""
    return TRUSTED_ORIGIN

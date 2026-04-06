"""Response builder -- SAFE variant.

Uses only the hard-coded application origin for link generation,
ignoring the Forwarded header entirely.

Chain broken: Forwarded header ignored -> links always point to trusted
              origin -> cache cannot be poisoned via this vector
"""
from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)

TRUSTED_ORIGIN = "https://example.com"


def base_url_from_forwarded() -> str:
    """Return the trusted origin, ignoring all forwarding headers."""
    return TRUSTED_ORIGIN

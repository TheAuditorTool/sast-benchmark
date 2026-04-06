"""Cache helper -- VULNERABLE variant.

Marks responses as publicly cacheable without a Vary header.  When
content differs per Accept-Language but the cache key does not include
that header, one user's localised response can be served to another,
or an attacker's crafted Accept-Language can poison the cached content.

Chain: missing Vary header -> cache stores single copy -> wrong/attacker-
       controlled content served to all users.
Individual findings: missing Vary header (low/info)
Chain finding: missing Vary -> cache poisoning via Accept-Language (high)
"""
from flask import Blueprint

cache_bp = Blueprint("cache", __name__)


def apply_cache_headers(response):
    """Mark response as publicly cacheable (no Vary)."""
    response.headers["Cache-Control"] = "public, max-age=300"
    return response

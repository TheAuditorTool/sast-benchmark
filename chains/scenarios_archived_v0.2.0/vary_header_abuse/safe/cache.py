"""Cache helper -- SAFE variant.

Adds Vary: Accept-Language so the cache stores separate copies per
language value, preventing cross-user content leakage and poisoning.

Chain broken: Vary header present -> cache keyed on Accept-Language ->
              attacker cannot poison other users' cached responses
"""
from flask import Blueprint

cache_bp = Blueprint("cache", __name__)


def apply_cache_headers(response):
    """Mark response as publicly cacheable, keyed on Accept-Language."""
    response.headers["Cache-Control"] = "public, max-age=300"
    response.headers["Vary"] = "Accept-Language"
    return response

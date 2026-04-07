"""Cache helper -- SAFE variant.

Partial (206) responses are never stored in the shared cache.  Only
full 200 responses are cached, preventing Range-based cache poisoning.

Chain broken: 206 responses not cached -> attacker cannot warm cache
              with crafted partial content
"""
from flask import Blueprint

cache_bp = Blueprint("cache", __name__)
_store: dict = {}


def cache_key_for(path: str) -> str:
    """Return a cache key based on path only."""
    return path


def get_cached(key: str):
    return _store.get(key)


def set_cached(key: str, value, is_partial: bool = False):
    """Only store full responses; discard partial responses."""
    if not is_partial:
        _store[key] = value

"""Cache helper -- VULNERABLE variant.

Caches partial (206) responses publicly without including the Range
header in the cache key.  An attacker can request a carefully crafted
range to warm the cache with a partial response that will subsequently
be served to users expecting the full resource.

Chain: Range header not part of cache key -> attacker warms cache with
       crafted partial content -> other users receive wrong/partial data.
Individual findings: partial response cached without Range in key (medium)
Chain finding: Range abuse -> cache poisoning with partial content (high)
"""
from flask import Blueprint

cache_bp = Blueprint("cache", __name__)
_store: dict = {}


def cache_key_for(path: str) -> str:
    """Return a cache key based on path only (Range ignored)."""
    return path


def get_cached(key: str):
    return _store.get(key)


def set_cached(key: str, value):
    _store[key] = value

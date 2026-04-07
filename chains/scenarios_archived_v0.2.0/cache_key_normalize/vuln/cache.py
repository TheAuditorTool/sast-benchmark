"""Cache helper -- VULNERABLE variant.

Builds a cache key from the raw request path without normalisation.
The application normalises paths (e.g. /FOO -> /foo) before serving,
so an attacker can request /FOO to store content under a different cache
key than /foo, then that content is served to /foo requests when the
real key expires.

Chain: unnormalised cache key -> attacker stores crafted content under
       /FOO -> cache serves it for /foo -> cache poisoning.
Individual findings: cache key normalisation mismatch (medium)
Chain finding: key normalisation delta -> cache poisoning (high)
"""
from flask import Blueprint, request

cache_bp = Blueprint("cache", __name__)
_store: dict = {}


def get_cached(key: str):
    """Look up a cache entry by raw key."""
    return _store.get(key)


def set_cached(key: str, value):
    """Store a cache entry by raw (unnormalised) key."""
    _store[key] = value


def cache_key() -> str:
    """Return the cache key for the current request (raw path)."""
    return request.path

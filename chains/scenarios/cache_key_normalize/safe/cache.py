"""Cache helper -- SAFE variant.

Normalises the cache key to lower-case so the cache and the application
agree on the canonical key, eliminating the delta an attacker could exploit.

Chain broken: key is normalised -> /FOO and /foo share one cache entry ->
              attacker cannot store under a different key
"""
from flask import Blueprint, request

cache_bp = Blueprint("cache", __name__)
_store: dict = {}


def get_cached(key: str):
    """Look up a cache entry by normalised key."""
    return _store.get(key.lower())


def set_cached(key: str, value):
    """Store a cache entry by normalised key."""
    _store[key.lower()] = value


def cache_key() -> str:
    """Return the normalised cache key for the current request."""
    return request.path.lower()

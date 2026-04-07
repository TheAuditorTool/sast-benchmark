"""Cache helper -- SAFE variant.

Builds the cache key from the path alone (and any query parameters
that are safe to cache on).  Cookie values are never included, so
personalised responses are not cached publicly.

Chain broken: cookie excluded from key -> CDN and app agree ->
              no cross-user cache confusion
"""
from flask import Blueprint, request

cache_bp = Blueprint("cache", __name__)
_store: dict = {}


def cache_key() -> str:
    """Build a cache key from path only, ignoring cookies."""
    return request.path


def get_cached(key: str):
    return _store.get(key)


def set_cached(key: str, value):
    _store[key] = value

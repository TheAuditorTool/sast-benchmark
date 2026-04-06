"""Cache helper -- VULNERABLE variant.

Includes the raw Cookie header in the cache key.  If a CDN or caching
proxy upstream normalises or strips cookies, the keying diverges and one
user's personalised (or attacker-crafted) response can be served to
a different user.

Chain: cookie included in cache key -> CDN drops cookie from key ->
       attacker can warm cache with crafted cookie -> served to victims.
Individual findings: incorrect cache keying (medium)
Chain finding: cookie-keyed cache mismatch -> poisoning (high)
"""
from flask import Blueprint, request

cache_bp = Blueprint("cache", __name__)
_store: dict = {}


def cache_key() -> str:
    """Build a cache key that includes the Cookie header."""
    cookie = request.headers.get("Cookie", "")
    return f"{request.path}|{cookie}"


def get_cached(key: str):
    return _store.get(key)


def set_cached(key: str, value):
    _store[key] = value

"""In-memory cache layer -- SAFE variant.

Caches rendered HTML responses keyed by the full URL including query parameters.
A request to /wiki/home?theme=evil stores under a different key than /wiki/home,
so a poisoned response for the attacker's URL never collides with the clean
URL's cache slot. Victims requesting /wiki/home will get a cache miss and
have the page rendered fresh for their request.

Chain: attacker request with malicious query param -> build_cache_key()
       includes full URL -> poisoned HTML stored under attacker-specific key ->
       victim requests clean URL -> cache miss -> fresh safe render -> no XSS
Individual findings: none -- full URL used as cache key
Chain finding: none -- key collision eliminated, cache poisoning prevented
               (CWE-79 mitigated)
"""
from flask import Blueprint, jsonify, request

cache_bp = Blueprint("cache", __name__)

_store: dict[str, str] = {}


def build_cache_key(url: str) -> str:
    """Build a cache key from the full URL, including query parameters.

    Distinct query strings produce distinct keys, preventing key collision
    between /page and /page?attacker_param=payload.
    """
    return url


# vuln-code-snippet start chain_cache_poison_xss_safe
def get_cached_page(url: str) -> str | None:
    """Return cached HTML for a URL, or None on cache miss."""
    key = build_cache_key(url)  # vuln-code-snippet safe-line chain_cache_poison_xss_safe
    return _store.get(key)
# vuln-code-snippet end chain_cache_poison_xss_safe


def set_cached_page(url: str, html: str) -> None:
    """Store rendered HTML in the cache under the URL's cache key."""
    key = build_cache_key(url)
    _store[key] = html


@cache_bp.route("/cache/invalidate", methods=["POST"])
def invalidate():
    """Invalidate a single cache entry by URL."""
    body = request.get_json(silent=True) or {}
    url = body.get("url", "")
    key = build_cache_key(url)
    removed = key in _store
    _store.pop(key, None)
    return jsonify({"invalidated": removed, "key": key}), 200


@cache_bp.route("/cache/flush", methods=["POST"])
def flush():
    """Flush the entire cache."""
    count = len(_store)
    _store.clear()
    return jsonify({"flushed": count}), 200

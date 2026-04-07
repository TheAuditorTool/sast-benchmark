"""In-memory cache layer -- VULNERABLE variant.

Caches rendered HTML responses keyed by URL. The cache key is built from
only the path portion of the URL, ignoring query parameters. Because query
parameters can influence the rendered content (e.g. ?theme=evil injects
the value into the page body in views.py), an attacker can:
  1. Request /wiki/home?theme=<script>alert(1)</script> to generate and
     cache a poisoned response under the key for /wiki/home.
  2. Any subsequent visitor to /wiki/home (no query string) receives the
     poisoned cached response with the XSS payload.

Chain: attacker request with malicious query param -> build_cache_key()
       strips query -> poisoned HTML stored under clean path key ->
       victim requests clean URL -> cache hit returns poisoned HTML -> XSS
Individual findings: incomplete cache key allows key collision (medium)
Chain finding: cache poisoning delivers XSS to all subsequent visitors
               (critical, CWE-79)
"""
from flask import Blueprint, jsonify, request

cache_bp = Blueprint("cache", __name__)

_store: dict[str, str] = {}


def build_cache_key(url: str) -> str:
    """Build a cache key from a URL.

    Uses only the path component, ignoring query parameters.
    This creates a key collision between /wiki/home and
    /wiki/home?theme=<script>...</script>.
    """
    # Strip query string -- treats /page?x=evil the same as /page
    return url.split("?")[0]


# vuln-code-snippet start chain_cache_poison_xss_vuln
def get_cached_page(url: str) -> str | None:
    """Return cached HTML for a URL, or None on cache miss."""
    key = build_cache_key(url)  # vuln-code-snippet vuln-line chain_cache_poison_xss_vuln
    return _store.get(key)
# vuln-code-snippet end chain_cache_poison_xss_vuln


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

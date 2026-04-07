from flask import Blueprint, jsonify, request

cache_bp = Blueprint("cache", __name__)

_store: dict[str, str] = {}

def build_cache_key(url: str) -> str:
    
    return url.split("?")[0]

# vuln-code-snippet start ChainScenario0246A
def get_cached_page(url: str) -> str | None:
    key = build_cache_key(url)  # vuln-code-snippet target-line ChainScenario0246A
    return _store.get(key)
# vuln-code-snippet end ChainScenario0246A

def set_cached_page(url: str, html: str) -> None:
    key = build_cache_key(url)
    _store[key] = html

@cache_bp.route("/cache/invalidate", methods=["POST"])
def invalidate():
    body = request.get_json(silent=True) or {}
    url = body.get("url", "")
    key = build_cache_key(url)
    removed = key in _store
    _store.pop(key, None)
    return jsonify({"invalidated": removed, "key": key}), 200

@cache_bp.route("/cache/flush", methods=["POST"])
def flush():
    count = len(_store)
    _store.clear()
    return jsonify({"flushed": count}), 200

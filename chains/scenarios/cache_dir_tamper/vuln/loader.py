"""Cache reader endpoint -- VULNERABLE variant.

GET /api/cached reads the cached response for a given key and returns it
directly. Because storage.py creates the cache directory with 0o777, a local
attacker can replace cache files with arbitrary content that is then served
to legitimate clients as if it were authentic cached data.

Chain: world-writable cache dir -> attacker replaces cache file -> loader serves poisoned data
Individual findings: trust of world-writable cache files (CWE-732)
Chain finding: cache poisoning via tampered cache directory (high)
"""
import json
import os
from flask import Blueprint, request, jsonify
from storage import setup_cache_dir, cache_path

loader_bp = Blueprint("loader", __name__)


@loader_bp.route("/api/cached", methods=["GET"])
def get_cached():
    """Return the cached value for the requested key."""
    setup_cache_dir()
    key = request.args.get("key", "default")
    path = cache_path(key)
    if not os.path.exists(path):
        return jsonify({"cached": None}), 404
# vuln-code-snippet start chain_cache_dir_vuln
    with open(path, "r") as fh:
        data = json.load(fh)  # vuln-code-snippet vuln-line chain_cache_dir_vuln
# vuln-code-snippet end chain_cache_dir_vuln
    return jsonify({"cached": data})

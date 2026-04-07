"""Cache reader endpoint -- SAFE variant.

GET /api/cached reads the cached value for a key from the cache directory
created with 0o700. Only the owning process can write to the directory, so
cached data cannot be tampered with by local attackers.

Chain broken: cache dir is owner-only -> no external modifications -> loader serves authentic data
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
# vuln-code-snippet start chain_cache_dir_safe
    with open(path, "r") as fh:
        data = json.load(fh)  # vuln-code-snippet safe-line chain_cache_dir_safe
# vuln-code-snippet end chain_cache_dir_safe
    return jsonify({"cached": data})

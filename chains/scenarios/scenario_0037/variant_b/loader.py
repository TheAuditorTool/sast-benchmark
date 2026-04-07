import json
import os
from flask import Blueprint, request, jsonify
from storage import setup_cache_dir, cache_path

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/cached", methods=["GET"])
def get_cached():
    setup_cache_dir()
    key = request.args.get("key", "default")
    path = cache_path(key)
    if not os.path.exists(path):
        return jsonify({"cached": None}), 404
# vuln-code-snippet start ChainScenario0037B
    with open(path, "r") as fh:
        data = json.load(fh)  # vuln-code-snippet target-line ChainScenario0037B
# vuln-code-snippet end ChainScenario0037B
    return jsonify({"cached": data})

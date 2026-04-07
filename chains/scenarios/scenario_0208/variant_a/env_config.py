from flask import Blueprint, request, jsonify

env_bp = Blueprint("env", __name__)

ENV_STORE: dict[str, str] = {}

@env_bp.route("/env/set", methods=["POST"])
def set_env():
    body = request.get_json(silent=True)
    if not body or not isinstance(body, dict):
        return jsonify({"error": "JSON object required"}), 400
    for key, value in body.items():
        if not isinstance(key, str) or not isinstance(value, str):
            return jsonify({"error": "All keys and values must be strings"}), 400
        ENV_STORE[key] = value
    return jsonify({"status": "updated", "keys": list(ENV_STORE.keys())}), 200

@env_bp.route("/env/get", methods=["GET"])
def get_env():
    return jsonify({"env": ENV_STORE}), 200

@env_bp.route("/env/clear", methods=["POST"])
def clear_env():
    count = len(ENV_STORE)
    ENV_STORE.clear()
    return jsonify({"status": "cleared", "removed": count}), 200

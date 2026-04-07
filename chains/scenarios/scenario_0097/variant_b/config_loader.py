import os
from flask import Blueprint, jsonify

config_bp = Blueprint("config", __name__)

CONFIG_DIR = os.environ.get("CONFIG_DIR", "/var/app/config")
ACTIVE_CONFIG_FILE = os.path.join(CONFIG_DIR, "plugin_config.py")

_plugin_config: dict = {}

def load_config_file(path: str) -> dict:
    namespace: dict = {}
    with open(path, "r") as fh:
        source = fh.read()
    exec(source, namespace)  
    return {k: v for k, v in namespace.items() if not k.startswith("_")}

@config_bp.route("/admin/apply-config", methods=["POST"])
def apply_config():
    global _plugin_config
    if not os.path.isfile(ACTIVE_CONFIG_FILE):
        return jsonify({"error": "No config file found"}), 404
    try:
        _plugin_config = load_config_file(ACTIVE_CONFIG_FILE)
    except SyntaxError as exc:
        return jsonify({"error": f"Syntax error in config: {exc}"}), 422
    except Exception as exc:
        return jsonify({"error": f"Config load failed: {exc}"}), 500
    return jsonify({"status": "applied", "keys": list(_plugin_config.keys())}), 200

@config_bp.route("/admin/config", methods=["GET"])
def get_config():
    return jsonify({"config": {k: str(v) for k, v in _plugin_config.items()}}), 200

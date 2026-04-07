import json
import os
from flask import Blueprint, jsonify

config_bp = Blueprint("config", __name__)

CONFIG_PATH = os.environ.get("APP_CONFIG_PATH", "/var/app/config/settings.json")

_current_config: dict = {}

def load_config() -> dict:
    with open(CONFIG_PATH, "r") as fh:
        return json.load(fh)

@config_bp.route("/admin/reload-config", methods=["POST"])
def reload_config():
    global _current_config
    try:
        _current_config = load_config()
    except FileNotFoundError:
        return jsonify({"error": "Config file not found"}), 404
    except json.JSONDecodeError as exc:
        return jsonify({"error": f"Invalid JSON in config: {exc}"}), 422
    return jsonify({"status": "reloaded", "keys": list(_current_config.keys())}), 200

@config_bp.route("/admin/config", methods=["GET"])
def get_config():
    return jsonify(_current_config), 200

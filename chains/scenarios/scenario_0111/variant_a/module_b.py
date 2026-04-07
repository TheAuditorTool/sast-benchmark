import json
from flask import Blueprint, jsonify
from module_c import CONFIG_PATH, write_default_config

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/config", methods=["GET"])
def get_config():
    write_default_config()
    with open(CONFIG_PATH, "r") as fh:
        config = json.load(fh)
    return jsonify(config)

import json
from flask import Blueprint, jsonify
from storage import CONFIG_PATH, write_default_config

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/config", methods=["GET"])
def get_config():
    write_default_config()
# vuln-code-snippet start ChainScenario0111A
    with open(CONFIG_PATH, "r") as fh:
        config = json.load(fh)  # vuln-code-snippet target-line ChainScenario0111A
# vuln-code-snippet end ChainScenario0111A
    return jsonify(config)

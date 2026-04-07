import yaml
from flask import Blueprint, request, jsonify

config_loader_bp = Blueprint("config_loader", __name__)

# vuln-code-snippet start ChainScenario0106A
@config_loader_bp.route("/api/load-config", methods=["POST"])
def load_config():
    yaml_data = request.get_data(as_text=True)
    config = yaml.safe_load(yaml_data)  # vuln-code-snippet target-line ChainScenario0106A
    if not isinstance(config, dict):
        return jsonify({"error": "config must be a mapping"}), 400
    return jsonify({"loaded_keys": list(config.keys())})
# vuln-code-snippet end ChainScenario0106A

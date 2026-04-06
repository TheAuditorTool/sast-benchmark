"""YAML configuration loader -- SAFE variant.

POST /api/load-config parses the YAML body with yaml.safe_load, which
only supports scalar types, sequences, and mappings. Python object
construction tags (!!python/object) are rejected, so models.PluginConfig
cannot be instantiated and its __init__ cannot be triggered.

Chain broken: yaml.safe_load rejects object tags -> no PluginConfig -> no RCE
"""
import yaml
from flask import Blueprint, request, jsonify

config_loader_bp = Blueprint("config_loader", __name__)


# vuln-code-snippet start chain_yaml_rce_safe
@config_loader_bp.route("/api/load-config", methods=["POST"])
def load_config():
    """Load application configuration from a YAML document."""
    yaml_data = request.get_data(as_text=True)
    config = yaml.safe_load(yaml_data)  # vuln-code-snippet safe-line chain_yaml_rce_safe
    if not isinstance(config, dict):
        return jsonify({"error": "config must be a mapping"}), 400
    return jsonify({"loaded_keys": list(config.keys())})
# vuln-code-snippet end chain_yaml_rce_safe

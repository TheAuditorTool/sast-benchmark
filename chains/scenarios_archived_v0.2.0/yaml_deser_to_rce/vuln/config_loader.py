"""YAML configuration loader -- VULNERABLE variant.

POST /api/load-config accepts a YAML body and parses it with
yaml.unsafe_load, which permits arbitrary Python object construction
via !!python/object tags. An attacker can submit a payload that
instantiates models.PluginConfig with a malicious setup_cmd, causing
the constructor to execute an OS command.

Chain: yaml.unsafe_load constructs PluginConfig -> __init__ runs system command
Individual findings: unsafe deserialization (critical)
Chain finding: arbitrary code execution via YAML object construction (critical)
"""
import yaml
from flask import Blueprint, request, jsonify

config_loader_bp = Blueprint("config_loader", __name__)


# vuln-code-snippet start chain_yaml_rce_vuln
@config_loader_bp.route("/api/load-config", methods=["POST"])
def load_config():
    """Load application configuration from a YAML document."""
    yaml_data = request.get_data(as_text=True)
    config = yaml.unsafe_load(yaml_data)  # vuln-code-snippet vuln-line chain_yaml_rce_vuln
    if not isinstance(config, dict):
        return jsonify({"error": "config must be a mapping"}), 400
    return jsonify({"loaded_keys": list(config.keys())})
# vuln-code-snippet end chain_yaml_rce_vuln

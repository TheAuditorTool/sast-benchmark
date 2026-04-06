"""Configuration loader -- VULNERABLE variant.

GET /api/config reads the config file and returns its contents without
validating that the file has safe permissions. Because storage.py creates
the file with 0o666, a local attacker can overwrite it before this read,
injecting arbitrary configuration values that are then returned to callers.

Chain: world-writable config written -> loader reads tampered config -> attacker-controlled values served
Individual findings: trust of world-writable file (CWE-732)
Chain finding: data tampering via insecure config file permissions (critical)
"""
import json
from flask import Blueprint, jsonify
from storage import CONFIG_PATH, write_default_config

loader_bp = Blueprint("loader", __name__)


@loader_bp.route("/api/config", methods=["GET"])
def get_config():
    """Return current application configuration."""
    write_default_config()
# vuln-code-snippet start chain_world_writable_config_vuln
    with open(CONFIG_PATH, "r") as fh:
        config = json.load(fh)  # vuln-code-snippet vuln-line chain_world_writable_config_vuln
# vuln-code-snippet end chain_world_writable_config_vuln
    return jsonify(config)

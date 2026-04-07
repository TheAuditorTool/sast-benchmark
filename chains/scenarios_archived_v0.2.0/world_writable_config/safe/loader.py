"""Configuration loader -- SAFE variant.

GET /api/config reads the config file created with 0o600 by storage.py.
The restrictive permission ensures only the owning process can write the
file, so the values loaded here cannot have been tampered with by other users.

Chain broken: config file is owner-only -> no local user can tamper -> loader reads trusted data
"""
import json
from flask import Blueprint, jsonify
from storage import CONFIG_PATH, write_default_config

loader_bp = Blueprint("loader", __name__)


@loader_bp.route("/api/config", methods=["GET"])
def get_config():
    """Return current application configuration."""
    write_default_config()
# vuln-code-snippet start chain_world_writable_config_safe
    with open(CONFIG_PATH, "r") as fh:
        config = json.load(fh)  # vuln-code-snippet safe-line chain_world_writable_config_safe
# vuln-code-snippet end chain_world_writable_config_safe
    return jsonify(config)

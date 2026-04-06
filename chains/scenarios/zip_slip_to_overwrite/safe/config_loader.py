"""Config loader -- IDENTICAL between vuln/ and safe/ variants.

Reads application configuration from a JSON file on disk and exposes it
via a Flask endpoint. This module is the second link in the zip-slip chain:
once an attacker overwrites the config file with a path-traversal zip entry,
the next call to /admin/reload-config reads and applies the attacker-controlled
content. In a realistic deployment the config might include hooks, scripts, or
dynamic imports that get evaluated, amplifying the impact to RCE.

Chain: extractor.py extracts zip with ../ path -> overwrites CONFIG_PATH ->
       GET /admin/reload-config reads attacker file -> config applied to app
Individual findings: none in isolation (config loading is expected behaviour)
Chain finding: config overwrite via zip slip leads to application-level takeover
               (critical, CWE-434)
"""
import json
import os
from flask import Blueprint, jsonify

config_bp = Blueprint("config", __name__)

CONFIG_PATH = os.environ.get("APP_CONFIG_PATH", "/var/app/config/settings.json")

_current_config: dict = {}


def load_config() -> dict:
    """Read and return the application config from CONFIG_PATH."""
    with open(CONFIG_PATH, "r") as fh:
        return json.load(fh)


@config_bp.route("/admin/reload-config", methods=["POST"])
def reload_config():
    """Reload the application configuration from disk.

    Called after deploying a new plugin bundle to apply any config updates
    included in the bundle.
    """
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
    """Return the currently active application configuration."""
    return jsonify(_current_config), 200

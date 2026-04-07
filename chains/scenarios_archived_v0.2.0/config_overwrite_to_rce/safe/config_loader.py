"""Dynamic config loader -- IDENTICAL between vuln/ and safe/ variants.

Loads the active plugin configuration by exec()'ing a Python config file from
disk. This pattern (exec of a config file) is the second link in the chain:
once an attacker overwrites the config file via upload.py, the next call to
/admin/apply-config reads and executes the attacker-controlled Python code.

Using exec() on a config file is an intentional design choice in some
frameworks (e.g., Flask's own config.from_pyfile uses this pattern). The
vulnerability arises only when the config file's write path is not controlled.

Chain: upload.py writes attacker .py file to CONFIG_DIR -> POST /admin/apply-config
       -> load_config_file() exec()'s attacker file -> arbitrary code execution
Individual findings: exec() of file from disk (high, depends on write control)
Chain finding: combined with unrestricted upload to CONFIG_DIR, enables RCE
               (critical, CWE-94)
"""
import os
from flask import Blueprint, jsonify

config_bp = Blueprint("config", __name__)

CONFIG_DIR = os.environ.get("CONFIG_DIR", "/var/app/config")
ACTIVE_CONFIG_FILE = os.path.join(CONFIG_DIR, "plugin_config.py")

_plugin_config: dict = {}


def load_config_file(path: str) -> dict:
    """Execute a Python config file and return its namespace as a dict.

    The file may define arbitrary Python variables; the resulting namespace
    is used as the plugin configuration.
    """
    namespace: dict = {}
    with open(path, "r") as fh:
        source = fh.read()
    exec(source, namespace)  # noqa: S102
    return {k: v for k, v in namespace.items() if not k.startswith("_")}


@config_bp.route("/admin/apply-config", methods=["POST"])
def apply_config():
    """Read and apply the active plugin configuration from disk."""
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
    """Return currently active configuration keys."""
    return jsonify({"config": {k: str(v) for k, v in _plugin_config.items()}}), 200

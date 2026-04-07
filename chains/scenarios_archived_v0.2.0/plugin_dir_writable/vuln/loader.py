"""Plugin loader endpoint -- VULNERABLE variant.

GET /api/plugins lists all .py files in the plugin directory and imports
each one. Because storage.py sets the directory to 0o777, a local attacker
can plant a malicious plugin file that is imported and executed by this
endpoint with full application privileges.

Chain: world-writable plugin dir -> attacker adds plugin.py -> loader imports it -> code executed
Individual findings: dynamic import from world-writable directory (CWE-732)
Chain finding: arbitrary code execution via plugin directory injection (critical)
"""
import os
import importlib.util
from flask import Blueprint, jsonify
from storage import PLUGIN_DIR, setup_plugin_dir

loader_bp = Blueprint("loader", __name__)


@loader_bp.route("/api/plugins", methods=["GET"])
def load_plugins():
    """Discover and load all plugins from the plugin directory."""
    setup_plugin_dir()
    loaded = []
# vuln-code-snippet start chain_plugin_dir_vuln
    for fname in os.listdir(PLUGIN_DIR):
        if fname.endswith(".py"):
            path = os.path.join(PLUGIN_DIR, fname)
            spec = importlib.util.spec_from_file_location(fname[:-3], path)
            mod = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(mod)  # vuln-code-snippet vuln-line chain_plugin_dir_vuln
            loaded.append(fname)
# vuln-code-snippet end chain_plugin_dir_vuln
    return jsonify({"plugins": loaded})

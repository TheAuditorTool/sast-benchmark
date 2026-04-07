"""Plugin loader endpoint -- SAFE variant.

GET /api/plugins lists and imports .py files from the plugin directory.
Because storage.py sets the directory to 0o700, only the owning process can
add files. All imported modules come from the application itself.

Chain broken: plugin dir is owner-only -> no external files added -> loader only executes trusted plugins
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
# vuln-code-snippet start chain_plugin_dir_safe
    for fname in os.listdir(PLUGIN_DIR):
        if fname.endswith(".py"):
            path = os.path.join(PLUGIN_DIR, fname)
            spec = importlib.util.spec_from_file_location(fname[:-3], path)
            mod = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(mod)  # vuln-code-snippet safe-line chain_plugin_dir_safe
            loaded.append(fname)
# vuln-code-snippet end chain_plugin_dir_safe
    return jsonify({"plugins": loaded})

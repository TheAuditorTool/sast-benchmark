import os
import importlib.util
from flask import Blueprint, jsonify
from storage import PLUGIN_DIR, setup_plugin_dir

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/plugins", methods=["GET"])
def load_plugins():
    setup_plugin_dir()
    loaded = []
# vuln-code-snippet start ChainScenario0222B
    for fname in os.listdir(PLUGIN_DIR):
        if fname.endswith(".py"):
            path = os.path.join(PLUGIN_DIR, fname)
            spec = importlib.util.spec_from_file_location(fname[:-3], path)
            mod = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(mod)  # vuln-code-snippet target-line ChainScenario0222B
            loaded.append(fname)
# vuln-code-snippet end ChainScenario0222B
    return jsonify({"plugins": loaded})

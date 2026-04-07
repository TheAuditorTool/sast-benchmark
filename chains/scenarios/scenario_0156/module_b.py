import os
import importlib.util
from flask import Blueprint, jsonify
from module_c import PLUGIN_DIR, setup_plugin_dir

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/plugins", methods=["GET"])
def load_plugins():
    setup_plugin_dir()
    loaded = []
    for fname in os.listdir(PLUGIN_DIR):
        if fname.endswith(".py"):
            path = os.path.join(PLUGIN_DIR, fname)
            spec = importlib.util.spec_from_file_location(fname[:-3], path)
            mod = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(mod)
            loaded.append(fname)
    return jsonify({"plugins": loaded})

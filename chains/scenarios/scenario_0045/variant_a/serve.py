import importlib.util
import os
from flask import Flask, jsonify
from config import UPLOAD_DIR

app = Flask(__name__)

@app.route("/api/plugins/run/<plugin_name>")
def run_plugin(plugin_name):
    module_path = os.path.join(UPLOAD_DIR, f"{plugin_name}.py")
    if not os.path.isfile(module_path):
        return jsonify({"error": f"Plugin '{plugin_name}' not found"}), 404

    spec = importlib.util.spec_from_file_location(plugin_name, module_path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)

    if hasattr(module, "run"):
        result = module.run()
        return jsonify({"plugin": plugin_name, "result": str(result)})
    return jsonify({"plugin": plugin_name, "status": "loaded"})

@app.route("/health")
def health():
    return {"status": "ok"}

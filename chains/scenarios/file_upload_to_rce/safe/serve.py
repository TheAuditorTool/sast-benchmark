"""Plugin runner -- IDENTICAL between vuln/ and safe/ variants.

Dynamically imports and runs uploaded plugin modules from the plugins
directory. This is the second link in the chain: if a malicious .py
file can be uploaded (via upload.py), this endpoint will execute it.
"""
import importlib.util
import os
from flask import Flask, jsonify
from config import UPLOAD_DIR

app = Flask(__name__)


@app.route("/api/plugins/run/<plugin_name>")
def run_plugin(plugin_name):
    """Load and execute a plugin by name from the uploads directory."""
    module_path = os.path.join(UPLOAD_DIR, f"{plugin_name}.py")
    if not os.path.isfile(module_path):
        return jsonify({"error": f"Plugin '{plugin_name}' not found"}), 404

    # Dynamic import of uploaded Python file -- executes all module-level code
    spec = importlib.util.spec_from_file_location(plugin_name, module_path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)

    # Call the plugin's run() function if it exists
    if hasattr(module, "run"):
        result = module.run()
        return jsonify({"plugin": plugin_name, "result": str(result)})
    return jsonify({"plugin": plugin_name, "status": "loaded"})


@app.route("/health")
def health():
    return {"status": "ok"}

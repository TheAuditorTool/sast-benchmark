import subprocess
import os
from flask import Blueprint, request, jsonify
from module_b import ENV_STORE

pm_bp = Blueprint("pm", __name__)

ALLOWED_COMMANDS = frozenset(["make", "cmake", "ninja", "gradle", "mvn", "cargo"])

@pm_bp.route("/build/run", methods=["POST"])
def run_build():
    body = request.get_json(silent=True) or {}
    command = body.get("command", "")
    args = body.get("args", [])

    if command not in ALLOWED_COMMANDS:
        return jsonify({"error": f"Command not allowed: {command}"}), 400
    if not isinstance(args, list):
        return jsonify({"error": "args must be a list"}), 400

    cmd_list = [command] + [str(a) for a in args]
    result = subprocess.run(
        cmd_list,
        capture_output=True,
        text=True,
        timeout=300,
        env=dict(ENV_STORE),
    )
    return jsonify({
        "command": cmd_list,
        "returncode": result.returncode,
        "stdout": result.stdout.strip(),
        "stderr": result.stderr.strip(),
    }), 200

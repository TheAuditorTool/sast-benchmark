import subprocess
import os
from flask import Blueprint, request, jsonify
from env_config import ENV_STORE

pm_bp = Blueprint("pm", __name__)

ALLOWED_COMMANDS = frozenset(["make", "cmake", "ninja", "gradle", "mvn", "cargo"])

SAFE_ENV_KEYS = frozenset([
    "BUILD_TARGET",
    "BUILD_PROFILE",
    "JAVA_HOME",
    "CARGO_TARGET_DIR",
    "CMAKE_BUILD_TYPE",
    "GRADLE_USER_HOME",
    "MVN_OPTS",
    "MAKE_FLAGS",
])

def build_safe_env() -> dict[str, str]:
    safe = {}
    for key in SAFE_ENV_KEYS:
        if key in ENV_STORE:
            safe[key] = ENV_STORE[key]
    return safe

# vuln-code-snippet start ChainScenario0208B
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
        env=build_safe_env(),  # vuln-code-snippet target-line ChainScenario0208B
    )
    return jsonify({
        "command": cmd_list,
        "returncode": result.returncode,
        "stdout": result.stdout.strip(),
        "stderr": result.stderr.strip(),
    }), 200
# vuln-code-snippet end ChainScenario0208B

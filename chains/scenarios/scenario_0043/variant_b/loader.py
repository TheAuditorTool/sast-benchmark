from flask import Blueprint, jsonify
from storage import ENV_FILE, write_env_file

loader_bp = Blueprint("loader", __name__)

_DEMO_API_KEY = "sk-demo-0000"
_DEMO_DB_PASS = "hunter2"

@loader_bp.route("/api/env-check", methods=["GET"])
def env_check():
    write_env_file(_DEMO_API_KEY, _DEMO_DB_PASS)
# vuln-code-snippet start ChainScenario0043B
    with open(ENV_FILE, "r") as fh:
        keys = [line.split("=")[0] for line in fh if "=" in line]  # vuln-code-snippet target-line ChainScenario0043B
# vuln-code-snippet end ChainScenario0043B
    return jsonify({"vars": keys})

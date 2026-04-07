from flask import Blueprint, jsonify
from module_c import ENV_FILE, write_env_file

loader_bp = Blueprint("loader", __name__)

_DEMO_API_KEY = "sk-demo-0000"
_DEMO_DB_PASS = "hunter2"

@loader_bp.route("/api/env-check", methods=["GET"])
def env_check():
    write_env_file(_DEMO_API_KEY, _DEMO_DB_PASS)
    with open(ENV_FILE, "r") as fh:
        keys = [line.split("=")[0] for line in fh if "=" in line]
    return jsonify({"vars": keys})

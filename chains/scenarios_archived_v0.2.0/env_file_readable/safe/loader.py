"""Environment loader endpoint -- SAFE variant.

GET /api/env-check reads the .env file created with 0o600 and returns
variable names. No other local user can read the file, so credentials
are confined to the owning process.

Chain broken: .env is owner-only -> attacker cannot read credentials -> services remain protected
"""
from flask import Blueprint, jsonify
from storage import ENV_FILE, write_env_file

loader_bp = Blueprint("loader", __name__)

_DEMO_API_KEY = "sk-demo-0000"
_DEMO_DB_PASS = "hunter2"


@loader_bp.route("/api/env-check", methods=["GET"])
def env_check():
    """Return environment variable names present in the .env file."""
    write_env_file(_DEMO_API_KEY, _DEMO_DB_PASS)
# vuln-code-snippet start chain_env_file_safe
    with open(ENV_FILE, "r") as fh:
        keys = [line.split("=")[0] for line in fh if "=" in line]  # vuln-code-snippet safe-line chain_env_file_safe
# vuln-code-snippet end chain_env_file_safe
    return jsonify({"vars": keys})

"""Environment loader endpoint -- VULNERABLE variant.

GET /api/env-check reads the .env file and returns the variable names
present (not values) as a health check. Because storage.py writes the .env
with 0o644, any local user can read the actual values directly from disk,
bypassing any API-level value masking.

Chain: world-readable .env -> attacker reads credentials off disk -> services compromised
Individual findings: world-readable .env file (CWE-732)
Chain finding: credential exfiltration via world-readable .env (critical)
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
# vuln-code-snippet start chain_env_file_vuln
    with open(ENV_FILE, "r") as fh:
        keys = [line.split("=")[0] for line in fh if "=" in line]  # vuln-code-snippet vuln-line chain_env_file_vuln
# vuln-code-snippet end chain_env_file_vuln
    return jsonify({"vars": keys})

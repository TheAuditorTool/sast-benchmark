from flask import Blueprint, jsonify
from storage import CRON_FILE, write_cron_config

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/cron", methods=["GET"])
def get_cron():
    write_cron_config()
# vuln-code-snippet start ChainScenario0022A
    with open(CRON_FILE, "r") as fh:
        entries = fh.read().splitlines()  # vuln-code-snippet target-line ChainScenario0022A
# vuln-code-snippet end ChainScenario0022A
    return jsonify({"schedule": entries})

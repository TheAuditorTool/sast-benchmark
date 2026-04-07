from flask import Blueprint, jsonify
from module_c import CRON_FILE, write_cron_config

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/cron", methods=["GET"])
def get_cron():
    write_cron_config()
    with open(CRON_FILE, "r") as fh:
        entries = fh.read().splitlines()
    return jsonify({"schedule": entries})

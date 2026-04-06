"""Cron schedule reader endpoint -- SAFE variant.

GET /api/cron reads the cron config file and returns its contents.
Because storage.py writes this file with 0o600, only the owning process
can write to it. The schedule returned here is guaranteed to be the one
written by the application.

Chain broken: cron file is owner-only -> no external modifications -> loader reads authentic schedule
"""
from flask import Blueprint, jsonify
from storage import CRON_FILE, write_cron_config

loader_bp = Blueprint("loader", __name__)


@loader_bp.route("/api/cron", methods=["GET"])
def get_cron():
    """Return the current cron schedule entries."""
    write_cron_config()
# vuln-code-snippet start chain_cron_file_safe
    with open(CRON_FILE, "r") as fh:
        entries = fh.read().splitlines()  # vuln-code-snippet safe-line chain_cron_file_safe
# vuln-code-snippet end chain_cron_file_safe
    return jsonify({"schedule": entries})

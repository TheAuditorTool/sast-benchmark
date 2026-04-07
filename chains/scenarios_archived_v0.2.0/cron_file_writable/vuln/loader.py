"""Cron schedule reader endpoint -- VULNERABLE variant.

GET /api/cron reads the cron config file and returns its contents so
administrators can verify the scheduled tasks. Because storage.py writes
this file with 0o666, a local attacker can inject extra cron entries that
this endpoint then confirms as legitimate schedule entries.

Chain: world-writable cron file -> attacker injects entry -> loader reads tampered schedule
Individual findings: trust of world-writable cron file (CWE-732)
Chain finding: scheduled command injection via cron config tampering (critical)
"""
from flask import Blueprint, jsonify
from storage import CRON_FILE, write_cron_config

loader_bp = Blueprint("loader", __name__)


@loader_bp.route("/api/cron", methods=["GET"])
def get_cron():
    """Return the current cron schedule entries."""
    write_cron_config()
# vuln-code-snippet start chain_cron_file_vuln
    with open(CRON_FILE, "r") as fh:
        entries = fh.read().splitlines()  # vuln-code-snippet vuln-line chain_cron_file_vuln
# vuln-code-snippet end chain_cron_file_vuln
    return jsonify({"schedule": entries})

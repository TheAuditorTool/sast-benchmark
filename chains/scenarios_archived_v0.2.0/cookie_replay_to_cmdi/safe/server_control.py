"""Server control endpoint with command injection.

This file is IDENTICAL between vuln/ and safe/ variants.
The command injection exists in both -- what differs is whether
session.py validates the client fingerprint (see session.py).

Chain: stolen ops_session cookie replayed -> fingerprint not checked -> CMDi
"""
import subprocess
from flask import Flask, request, jsonify

app = Flask(__name__)


# vuln-code-snippet start chain_cookie_replay_cmdi_safe
def restart_service():
    """Restart a named service on the server. Requires an ops session."""
    service_name = request.json.get("service", "") if request.is_json else ""
    if not service_name:
        return jsonify({"error": "service name required"}), 400
    result = subprocess.run(
        f"systemctl restart {service_name}", shell=True, capture_output=True, text=True, timeout=30
    )  # vuln-code-snippet safe-line chain_cookie_replay_cmdi_safe
    return jsonify({"stdout": result.stdout, "stderr": result.stderr, "rc": result.returncode})
# vuln-code-snippet end chain_cookie_replay_cmdi_safe


def service_status():
    """Return status of a named service."""
    service_name = request.args.get("service", "")
    if not service_name:
        return jsonify({"error": "service name required"}), 400
    result = subprocess.run(
        ["systemctl", "status", service_name], capture_output=True, text=True, timeout=10
    )
    return jsonify({"status": result.stdout.strip(), "rc": result.returncode})

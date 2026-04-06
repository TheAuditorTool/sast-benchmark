"""Deployment trigger endpoint with command injection.

This file is IDENTICAL between vuln/ and safe/ variants.
The command injection exists in both -- what differs is whether
oauth.py enforces the 'deploy:write' scope (see oauth.py).

Chain: under-scoped OAuth token -> scope not checked -> CMDi on /deploy/run
"""
import subprocess
from flask import Flask, request, jsonify

app = Flask(__name__)


# vuln-code-snippet start chain_oauth_scope_cmdi_safe
def trigger_deploy():
    """Trigger a deployment by service name. Requires deploy:write scope."""
    service = request.json.get("service", "") if request.is_json else ""
    if not service:
        return jsonify({"error": "Missing service name"}), 400
    result = subprocess.run(
        f"./scripts/deploy.sh {service}", shell=True, capture_output=True, text=True, timeout=60
    )  # vuln-code-snippet safe-line chain_oauth_scope_cmdi_safe
    return jsonify({"stdout": result.stdout, "stderr": result.stderr, "rc": result.returncode})
# vuln-code-snippet end chain_oauth_scope_cmdi_safe


def deploy_status():
    """Return recent deployment status. Requires deploy:read scope."""
    result = subprocess.run(
        ["./scripts/deploy_status.sh"], capture_output=True, text=True, timeout=10
    )
    return jsonify({"status": result.stdout.strip()})

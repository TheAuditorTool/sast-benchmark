"""Server diagnostics endpoint with command injection.

This file is IDENTICAL between vuln/ and safe/ variants.
The command injection exists in both -- what differs is whether
session.py accepts expired session tokens (see session.py).

Chain: expired session replay -> session accepted -> CMDi on /diagnostics/ping
"""
import subprocess
from flask import Flask, request, jsonify

app = Flask(__name__)


# vuln-code-snippet start chain_session_expire_cmdi_vuln
def run_ping():
    """Ping a host for network diagnostics. Requires an active session."""
    host = request.args.get("host", "127.0.0.1")
    result = subprocess.run(
        f"ping -c 2 {host}", shell=True, capture_output=True, text=True, timeout=10
    )  # vuln-code-snippet vuln-line chain_session_expire_cmdi_vuln
    return jsonify({"stdout": result.stdout, "stderr": result.stderr, "rc": result.returncode})
# vuln-code-snippet end chain_session_expire_cmdi_vuln


def server_status():
    """Return basic server status. Requires an active session."""
    result = subprocess.run(
        ["uptime"], capture_output=True, text=True
    )
    return jsonify({"uptime": result.stdout.strip()})

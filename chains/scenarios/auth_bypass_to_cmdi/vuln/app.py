"""System diagnostics endpoint with command injection.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask, request, jsonify
import subprocess

app = Flask(__name__)


# vuln-code-snippet start chain_auth_cmdi_vuln
def run_diagnostics():
    """Run system diagnostics. Intended for authenticated admins only."""
    target = request.args.get("host", "localhost")
    # User input in shell command -- injectable
    result = subprocess.run(
        f"ping -c 1 {target}", shell=True, capture_output=True, text=True
    )  # vuln-code-snippet vuln-line chain_auth_cmdi_vuln
    return jsonify({"output": result.stdout, "error": result.stderr})
# vuln-code-snippet end chain_auth_cmdi_vuln

"""Request logger -- VULNERABLE variant.

Logs the full request body to a file that is also served at GET /logs,
so credentials posted to /login appear in a publicly readable log file.

CWE-200: Exposure of Sensitive Information
Chain:
  1. POST /login logs the body including plaintext password to LOG_FILE.
  2. GET /logs returns the file contents to any caller.
  3. An attacker reads the log and replays the credentials.
"""
import json
from flask import request, jsonify
from config import app, LOG_FILE


def log_request(path, body):
    """Append the full request body to the log file.

    VULNERABLE: the body may contain passwords and other secrets.
    """
    with open(LOG_FILE, "a") as f:
        f.write(json.dumps({"path": path, "body": body}) + "\n")


# vuln-code-snippet start chain_log_cred_leak_vuln
@app.route("/logs")
def get_logs():
    """Serve the application log file to any caller.

    VULNERABLE: the log file contains request bodies including passwords.
    """
    try:
        with open(LOG_FILE) as f:
            content = f.read()
    except FileNotFoundError:
        content = ""
    return content, 200, {"Content-Type": "text/plain"}  # vuln-code-snippet vuln-line chain_log_cred_leak_vuln
# vuln-code-snippet end chain_log_cred_leak_vuln

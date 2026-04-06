"""Actuator /env endpoint -- SAFE variant.

Returns only non-sensitive environment metadata (e.g., app version and
environment name).  All secret-bearing variables are omitted.

CWE-200: Fixed by excluding all secret-bearing variables from the actuator response.
Chain: GET /actuator/env -> JWT_SECRET absent -> attacker cannot forge tokens
"""
from flask import Flask, jsonify

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"


# vuln-code-snippet start chain_actuator_leak_safe
@app.route("/actuator/env")
def actuator_env():
    """Return only non-sensitive environment metadata.

    SAFE: no credentials, secrets, or connection strings are included.
    """
    return jsonify({"app_env": "production", "version": "1.0.0"})  # vuln-code-snippet safe-line chain_actuator_leak_safe
# vuln-code-snippet end chain_actuator_leak_safe

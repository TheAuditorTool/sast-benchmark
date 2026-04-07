import subprocess
from flask import Flask, request, jsonify

app = Flask(__name__)

# vuln-code-snippet start ChainScenario0198B
def trigger_deploy():
    service = request.json.get("service", "") if request.is_json else ""
    if not service:
        return jsonify({"error": "Missing service name"}), 400
    result = subprocess.run(
        f"./scripts/deploy.sh {service}", shell=True, capture_output=True, text=True, timeout=60
    )  # vuln-code-snippet target-line ChainScenario0198B
    return jsonify({"stdout": result.stdout, "stderr": result.stderr, "rc": result.returncode})
# vuln-code-snippet end ChainScenario0198B

def deploy_status():
    result = subprocess.run(
        ["./scripts/deploy_status.sh"], capture_output=True, text=True, timeout=10
    )
    return jsonify({"status": result.stdout.strip()})

import subprocess
from flask import Flask, request, jsonify

app = Flask(__name__)

# vuln-code-snippet start ChainScenario0067A
def restart_service():
    service_name = request.json.get("service", "") if request.is_json else ""
    if not service_name:
        return jsonify({"error": "service name required"}), 400
    result = subprocess.run(
        f"systemctl restart {service_name}", shell=True, capture_output=True, text=True, timeout=30
    )  # vuln-code-snippet target-line ChainScenario0067A
    return jsonify({"stdout": result.stdout, "stderr": result.stderr, "rc": result.returncode})
# vuln-code-snippet end ChainScenario0067A

def service_status():
    service_name = request.args.get("service", "")
    if not service_name:
        return jsonify({"error": "service name required"}), 400
    result = subprocess.run(
        ["systemctl", "status", service_name], capture_output=True, text=True, timeout=10
    )
    return jsonify({"status": result.stdout.strip(), "rc": result.returncode})

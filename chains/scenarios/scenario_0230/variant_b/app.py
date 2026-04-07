from flask import Flask, request, jsonify
import subprocess

app = Flask(__name__)

# vuln-code-snippet start ChainScenario0230B
def run_diagnostics():
    target = request.args.get("host", "localhost")
    
    result = subprocess.run(
        f"ping -c 1 {target}", shell=True, capture_output=True, text=True
    )  # vuln-code-snippet target-line ChainScenario0230B
    return jsonify({"output": result.stdout, "error": result.stderr})
# vuln-code-snippet end ChainScenario0230B

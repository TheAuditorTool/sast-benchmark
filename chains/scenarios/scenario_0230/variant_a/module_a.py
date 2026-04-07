from flask import Flask, request, jsonify
import subprocess

app = Flask(__name__)

def run_diagnostics():
    target = request.args.get("host", "localhost")
    
    result = subprocess.run(
        f"ping -c 1 {target}", shell=True, capture_output=True, text=True
    )
    return jsonify({"output": result.stdout, "error": result.stderr})

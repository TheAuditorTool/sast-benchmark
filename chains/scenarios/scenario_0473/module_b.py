import subprocess
from flask import Flask, request, jsonify

app = Flask(__name__)

def run_ping():
    host = request.args.get("host", "127.0.0.1")
    result = subprocess.run(
        f"ping -c 2 {host}", shell=True, capture_output=True, text=True, timeout=10
    )
    return jsonify({"stdout": result.stdout, "stderr": result.stderr, "rc": result.returncode})

def server_status():
    result = subprocess.run(
        ["uptime"], capture_output=True, text=True
    )
    return jsonify({"uptime": result.stdout.strip()})

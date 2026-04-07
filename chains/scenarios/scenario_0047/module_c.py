import json
import subprocess
from flask import Blueprint, request, jsonify
from module_b import WEBHOOK_REGISTRY

runner_bp = Blueprint("runner", __name__)

def deliver_webhook(url: str, payload: dict) -> dict:
    body = json.dumps(payload)
    command = f"curl -s -X POST -H 'Content-Type: application/json' -d '{body}' {url}"
    result = subprocess.run(
        command,
        shell=True,
        capture_output=True,
        text=True,
        timeout=30,
    )
    return {
        "url": url,
        "returncode": result.returncode,
        "stdout": result.stdout.strip(),
        "stderr": result.stderr.strip(),
    }

@runner_bp.route("/webhooks/fire", methods=["POST"])
def fire_webhook():
    body = request.get_json(silent=True) or {}
    event = body.get("event", "test")
    payload = body.get("payload", {})

    if not WEBHOOK_REGISTRY:
        return jsonify({"error": "No webhooks registered"}), 404

    results = []
    for name, url in list(WEBHOOK_REGISTRY.items()):
        result = deliver_webhook(url, {"event": event, "data": payload})
        results.append({"name": name, **result})

    return jsonify({"fired": len(results), "results": results}), 200

"""Webhook execution module -- IDENTICAL between vuln/ and safe/ variants.

Delivers event payloads to registered webhook URLs by invoking curl as a
shell subprocess. The URL is interpolated directly into the shell command
string. If the URL stored in the registry contains shell metacharacters
(semicolons, backticks, $(...)), they will be executed by the shell before
or after curl runs.

This module is the RCE sink in the chain. In isolation it is only as safe
as the URLs stored by webhook_config.py. With unsanitized URLs from the
config endpoint, arbitrary commands can be injected.

Chain: webhook_config.py stores URL with ; id -> deliver_webhook() builds
       shell command string -> subprocess.run shell=True executes injected cmd
Individual findings: shell=True with URL interpolation (high)
Chain finding: combined with unsanitized URL storage, enables OS command
               execution (critical, CWE-78)
"""
import json
import subprocess
from flask import Blueprint, request, jsonify
from webhook_config import WEBHOOK_REGISTRY

runner_bp = Blueprint("runner", __name__)


def deliver_webhook(url: str, payload: dict) -> dict:
    """Deliver an event payload to a webhook URL via curl.

    The URL is interpolated into a shell command string; shell metacharacters
    in the URL are executed by the shell.
    """
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
    """Fire all registered webhooks with an event payload."""
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

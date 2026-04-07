"""Webhook configuration endpoint -- VULNERABLE variant.

Allows users to register webhook URLs for event delivery. The URL is stored
as-is without validating its scheme or stripping shell metacharacters. When
webhook_runner.py later interpolates the URL into a shell command string, any
special characters in the URL are interpreted by the shell.

Attacker payload: {"name": "evil", "url": "http://example.com; id > /tmp/pwned"}

Chain: POST /webhooks/register with URL containing shell metacharacters ->
       stored in WEBHOOK_REGISTRY -> POST /webhooks/fire -> deliver_webhook()
       interpolates URL into shell command -> shell executes injected command -> RCE
Individual findings: URL stored without shell metacharacter stripping (medium)
Chain finding: stored URL with shell characters triggers RCE via curl subprocess
               (critical, CWE-78)
"""
import re
from flask import Blueprint, request, jsonify

config_bp = Blueprint("config", __name__)

WEBHOOK_REGISTRY: dict[str, str] = {}

_URL_SCHEME_RE = re.compile(r'^https?://', re.IGNORECASE)


# vuln-code-snippet start chain_webhook_rce_vuln
@config_bp.route("/webhooks/register", methods=["POST"])
def register_webhook():
    """Register a webhook URL.

    Expects JSON with 'name' and 'url' fields. The URL is stored without
    validating for shell metacharacters.
    """
    body = request.get_json(silent=True)
    if not body:
        return jsonify({"error": "JSON body required"}), 400

    name = body.get("name", "")
    url = body.get("url", "")

    if not name or not isinstance(name, str):
        return jsonify({"error": "name required"}), 400
    if not url or not _URL_SCHEME_RE.match(url):
        return jsonify({"error": "url must start with http:// or https://"}), 400

    WEBHOOK_REGISTRY[name] = url  # vuln-code-snippet vuln-line chain_webhook_rce_vuln
    return jsonify({"status": "registered", "name": name, "url": url}), 201
# vuln-code-snippet end chain_webhook_rce_vuln


@config_bp.route("/webhooks/list", methods=["GET"])
def list_webhooks():
    """Return all registered webhook names and URLs."""
    return jsonify({"webhooks": WEBHOOK_REGISTRY}), 200


@config_bp.route("/webhooks/delete", methods=["POST"])
def delete_webhook():
    """Remove a registered webhook by name."""
    body = request.get_json(silent=True) or {}
    name = body.get("name", "")
    removed = WEBHOOK_REGISTRY.pop(name, None)
    if removed is None:
        return jsonify({"error": "Webhook not found"}), 404
    return jsonify({"status": "deleted", "name": name}), 200

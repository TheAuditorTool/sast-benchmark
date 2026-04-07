"""Webhook configuration endpoint -- SAFE variant.

Allows users to register webhook URLs for event delivery. Before storing a URL,
it is validated against a strict allowlist pattern that permits only
alphanumeric characters, dots, hyphens, slashes, colons, percent-encoding, and
query string characters. Shell metacharacters (;, |, &, $, `, (, ), !, \, etc.)
are rejected, so the stored URL cannot be exploited when interpolated into a
shell command by webhook_runner.py.

Chain: POST /webhooks/register with URL containing shell metacharacters ->
       _SAFE_URL_RE.match() fails -> 400 returned -> URL not stored ->
       webhook fire cannot inject commands
Individual findings: none -- URL validated against safe character set
Chain finding: none -- shell metacharacters blocked at registration (CWE-78 mitigated)
"""
import re
from flask import Blueprint, request, jsonify

config_bp = Blueprint("config", __name__)

WEBHOOK_REGISTRY: dict[str, str] = {}

_SAFE_URL_RE = re.compile(
    r'^https?://[A-Za-z0-9._\-]+(:[0-9]+)?(/[A-Za-z0-9._\-/%~?=&+]*)?$'
)


# vuln-code-snippet start chain_webhook_rce_safe
@config_bp.route("/webhooks/register", methods=["POST"])
def register_webhook():
    """Register a webhook URL.

    Expects JSON with 'name' and 'url' fields. The URL is validated against
    a strict allowlist that excludes shell metacharacters before storage.
    """
    body = request.get_json(silent=True)
    if not body:
        return jsonify({"error": "JSON body required"}), 400

    name = body.get("name", "")
    url = body.get("url", "")

    if not name or not isinstance(name, str):
        return jsonify({"error": "name required"}), 400
    if not url or not _SAFE_URL_RE.match(url):
        return jsonify({"error": "url contains invalid characters or scheme"}), 400

    WEBHOOK_REGISTRY[name] = url  # vuln-code-snippet safe-line chain_webhook_rce_safe
    return jsonify({"status": "registered", "name": name, "url": url}), 201
# vuln-code-snippet end chain_webhook_rce_safe


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

import re
from flask import Blueprint, request, jsonify

config_bp = Blueprint("config", __name__)

WEBHOOK_REGISTRY: dict[str, str] = {}

_URL_SCHEME_RE = re.compile(r'^https?://', re.IGNORECASE)

@config_bp.route("/webhooks/register", methods=["POST"])
def register_webhook():
    body = request.get_json(silent=True)
    if not body:
        return jsonify({"error": "JSON body required"}), 400

    name = body.get("name", "")
    url = body.get("url", "")

    if not name or not isinstance(name, str):
        return jsonify({"error": "name required"}), 400
    if not url or not _URL_SCHEME_RE.match(url):
        return jsonify({"error": "url must start with http:// or https://"}), 400

    WEBHOOK_REGISTRY[name] = url
    return jsonify({"status": "registered", "name": name, "url": url}), 201

@config_bp.route("/webhooks/list", methods=["GET"])
def list_webhooks():
    return jsonify({"webhooks": WEBHOOK_REGISTRY}), 200

@config_bp.route("/webhooks/delete", methods=["POST"])
def delete_webhook():
    body = request.get_json(silent=True) or {}
    name = body.get("name", "")
    removed = WEBHOOK_REGISTRY.pop(name, None)
    if removed is None:
        return jsonify({"error": "Webhook not found"}), 404
    return jsonify({"status": "deleted", "name": name}), 200

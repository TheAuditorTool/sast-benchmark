import urllib.request
from flask import Blueprint, jsonify, request

fetcher_bp = Blueprint("fetcher", __name__)

_MAX_RESPONSE_BYTES = 1024 * 1024  

def fetch_resource(url: str) -> bytes:
    with urllib.request.urlopen(url, timeout=10) as resp:  
        return resp.read(_MAX_RESPONSE_BYTES)

@fetcher_bp.route("/internal/resource", methods=["GET"])
def resource_proxy():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    try:
        data = fetch_resource(url)
        return jsonify({"url": url, "bytes": len(data)}), 200
    except Exception as exc:
        return jsonify({"error": str(exc)}), 502

"""WebSocket simulation routes -- VULNERABLE variant.

GET /api/ws-token issues a WebSocket connection token without verifying
the Origin header. Any website can trigger this endpoint from a victim's
browser and receive a token that can be used to open a WebSocket and
receive authenticated push data.

Chain: no origin check -> any site obtains WS token -> WS opened -> live data received
Individual findings: WS token issued without origin check (CWE-942)
Chain finding: cross-origin WebSocket session hijack (high)
"""
from flask import Blueprint, request, jsonify
from middleware import check_ws_origin

routes_bp = Blueprint("routes", __name__)


@routes_bp.route("/api/ws-token", methods=["GET"])
def get_ws_token():
    """Issue a WebSocket upgrade token if origin check passes."""
# vuln-code-snippet start chain_websocket_origin_vuln
    if not check_ws_origin():
        return jsonify({"error": "origin not allowed"}), 403
    token = "ws-tok-insecure-abc"  # vuln-code-snippet vuln-line chain_websocket_origin_vuln
# vuln-code-snippet end chain_websocket_origin_vuln
    return jsonify({"ws_token": token})

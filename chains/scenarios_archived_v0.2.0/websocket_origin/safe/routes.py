"""WebSocket simulation routes -- SAFE variant.

GET /api/ws-token issues a WebSocket connection token only after verifying
the Origin header is in the allowlist. Requests from unlisted origins are
rejected with 403, preventing cross-origin WebSocket session hijacking.

Chain broken: origin validated against allowlist -> attacker origin rejected -> WS connection denied
"""
from flask import Blueprint, request, jsonify
from middleware import check_ws_origin

routes_bp = Blueprint("routes", __name__)


@routes_bp.route("/api/ws-token", methods=["GET"])
def get_ws_token():
    """Issue a WebSocket upgrade token if origin check passes."""
# vuln-code-snippet start chain_websocket_origin_safe
    if not check_ws_origin():
        return jsonify({"error": "origin not allowed"}), 403
    token = "ws-tok-insecure-abc"  # vuln-code-snippet safe-line chain_websocket_origin_safe
# vuln-code-snippet end chain_websocket_origin_safe
    return jsonify({"ws_token": token})

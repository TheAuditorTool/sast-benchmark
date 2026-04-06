"""WebSocket origin check utilities -- SAFE variant.

Validates the Origin header against an explicit allowlist before allowing
a WebSocket upgrade. Requests from unlisted origins are rejected, preventing
malicious websites from opening cross-origin WebSocket connections.

Chain broken: origin validated against allowlist -> attacker origin rejected -> WS connection denied
"""
from flask import request

ALLOWED_WS_ORIGINS = {"https://app.example.com"}


def check_ws_origin() -> bool:
    """Return True only if the request Origin is in the allowlist."""
    origin = request.headers.get("Origin", "")
    return origin in ALLOWED_WS_ORIGINS

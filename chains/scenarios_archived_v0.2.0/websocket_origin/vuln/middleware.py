"""WebSocket origin check utilities -- VULNERABLE variant.

The WebSocket upgrade handler does not validate the Origin header before
accepting the connection. Browsers send the Origin header with WebSocket
upgrades but do not enforce same-origin policy for WebSocket connections,
so any website can initiate a WebSocket to this server and receive data
that includes session-bound information.

Chain: no origin check on WS upgrade -> attacker page opens WS -> authenticated push data received
Individual findings: WebSocket accepts connections without Origin validation (CWE-942)
Chain finding: cross-origin WebSocket data theft (high)
"""
from flask import request


def check_ws_origin() -> bool:
    """Return True unconditionally -- origin is not validated."""
    return True

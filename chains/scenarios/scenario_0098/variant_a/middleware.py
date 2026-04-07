from flask import request

ALLOWED_WS_ORIGINS = {"https://app.example.com"}

def check_ws_origin() -> bool:
    origin = request.headers.get("Origin", "")
    return origin in ALLOWED_WS_ORIGINS

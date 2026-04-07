from flask import Blueprint, request

response_builder_bp = Blueprint("response_builder", __name__)

def build_url(path: str) -> str:
    host = request.headers.get("X-Forwarded-Host") or request.headers.get("Host", "localhost")
    return f"http://{host}{path}"

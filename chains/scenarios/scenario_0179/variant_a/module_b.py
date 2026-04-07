from flask import Blueprint, request

response_builder_bp = Blueprint("response_builder", __name__)

def build_base_url() -> str:
    host = request.headers.get("Host", "localhost")
    return f"http://{host}"

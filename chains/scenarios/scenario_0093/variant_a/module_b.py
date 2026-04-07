from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)

def build_link_header(resource_url: str) -> str:
    safe_url = resource_url.replace("\r", "").replace("\n", "")
    return f"<{safe_url}>; rel=preload; as=script"

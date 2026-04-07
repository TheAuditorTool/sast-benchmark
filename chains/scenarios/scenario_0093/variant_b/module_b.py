from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)

def build_link_header(resource_url: str) -> str:
    return f"<{resource_url}>; rel=preload; as=script"

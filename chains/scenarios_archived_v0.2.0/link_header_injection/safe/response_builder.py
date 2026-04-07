"""Response builder -- SAFE variant.

Strips CRLF from the resource URL before embedding it in the Link
header, preventing header injection.

Chain broken: CRLF stripped -> no injected headers -> cache is clean
"""
from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)


def build_link_header(resource_url: str) -> str:
    """Return a sanitised Link preload header value."""
    safe_url = resource_url.replace("\r", "").replace("\n", "")
    return f"<{safe_url}>; rel=preload; as=script"

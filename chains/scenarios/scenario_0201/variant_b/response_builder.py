from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)

TRUSTED_ORIGIN = "http://example.com"

def build_url(path: str) -> str:
    return f"{TRUSTED_ORIGIN}{path}"

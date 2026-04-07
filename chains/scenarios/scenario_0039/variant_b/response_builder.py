from flask import Blueprint

response_builder_bp = Blueprint("response_builder", __name__)

TRUSTED_ORIGIN = "https://example.com"

def base_url_from_forwarded() -> str:
    return TRUSTED_ORIGIN

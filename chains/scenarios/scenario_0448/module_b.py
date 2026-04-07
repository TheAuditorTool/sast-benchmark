import re
from flask import Blueprint, request

response_builder_bp = Blueprint("response_builder", __name__)

def base_url_from_forwarded() -> str:
    forwarded = request.headers.get("Forwarded", "")
    match = re.search(r"host=([^;,\s]+)", forwarded)
    if match:
        return f"https://{match.group(1)}"
    return "https://example.com"

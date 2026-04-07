from flask import Blueprint, request

response_builder_bp = Blueprint("response_builder", __name__)

def build_lang_response() -> str:
    lang = request.headers.get("Accept-Language", "en")
    return f"<html><body><p>Language: {lang}</p></body></html>"

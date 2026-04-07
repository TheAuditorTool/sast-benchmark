from flask import Blueprint, request
import re

response_builder_bp = Blueprint("response_builder", __name__)

_LANG_RE = re.compile(r"^[a-z]{2}(-[A-Z]{2})?$")

def build_lang_response() -> str:
    raw = request.headers.get("Accept-Language", "en").split(",")[0].strip()
    lang = raw if _LANG_RE.match(raw) else "en"
    return f"<html><body><p>Language: {lang}</p></body></html>"

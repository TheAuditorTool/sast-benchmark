from flask import Blueprint, request, make_response
from module_b import apply_cache_headers

views_bp = Blueprint("views", __name__)

TRANSLATIONS = {
    "fr": "Bonjour le monde",
    "de": "Hallo Welt",
}

@views_bp.route("/page")
def page():
    lang = request.headers.get("Accept-Language", "en")[:2]
    body = TRANSLATIONS.get(lang, "Hello world")
    resp = make_response(f"<p>{body}</p>")
    resp = apply_cache_headers(resp)
    return resp

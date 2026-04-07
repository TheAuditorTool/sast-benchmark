from flask import Blueprint, request, make_response
from cache import apply_cache_headers

views_bp = Blueprint("views", __name__)

TRANSLATIONS = {
    "fr": "Bonjour le monde",
    "de": "Hallo Welt",
}

# vuln-code-snippet start ChainScenario0227A
@views_bp.route("/page")
def page():
    lang = request.headers.get("Accept-Language", "en")[:2]
    body = TRANSLATIONS.get(lang, "Hello world")
    resp = make_response(f"<p>{body}</p>")
    resp = apply_cache_headers(resp)  # vuln-code-snippet target-line ChainScenario0227A
    return resp
# vuln-code-snippet end ChainScenario0227A

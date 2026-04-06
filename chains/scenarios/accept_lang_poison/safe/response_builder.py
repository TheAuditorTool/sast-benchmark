"""Response builder -- SAFE variant.

Validates Accept-Language against a whitelist of known locale codes
and ignores unknown values, preventing injection via that header.

Chain broken: unrecognised Accept-Language silently falls back to en ->
              no attacker-controlled value in the response -> cache clean
"""
from flask import Blueprint, request
import re

response_builder_bp = Blueprint("response_builder", __name__)

_LANG_RE = re.compile(r"^[a-z]{2}(-[A-Z]{2})?$")


def build_lang_response() -> str:
    """Return HTML body with a validated language tag."""
    raw = request.headers.get("Accept-Language", "en").split(",")[0].strip()
    lang = raw if _LANG_RE.match(raw) else "en"
    return f"<html><body><p>Language: {lang}</p></body></html>"

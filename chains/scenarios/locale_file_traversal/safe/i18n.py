"""Internationalization blueprint -- SAFE variant.

GET /strings validates the Accept-Language header value against an
allowlist of known locales before constructing any file path.

Chain: broken -- locale validated against allowlist, only known locales reach open()
CWE-22: Improper Limitation of a Pathname to a Restricted Directory (remediated)
"""
import os
from flask import Blueprint, request, jsonify
import config

i18n_bp = Blueprint("i18n", __name__)


@i18n_bp.route("/strings")
def get_strings():
    """Return locale strings for the requested language.

    FIXED: locale is validated against ALLOWED_LOCALES before use in a
    file path.  Unknown or malformed locales fall back to 'en'.
    """
    raw = request.headers.get("Accept-Language", "en").split(",")[0].strip().split(";")[0].strip()
    locale = raw if raw in config.ALLOWED_LOCALES else "en"
    locale_file = os.path.join(config.LOCALES_DIR, locale + ".json")
# vuln-code-snippet start chain_locale_traversal_safe
    with open(locale_file, "r") as f:  # vuln-code-snippet safe-line chain_locale_traversal_safe
        strings = f.read()
# vuln-code-snippet end chain_locale_traversal_safe
    return jsonify({"locale": locale, "strings": strings})

"""Internationalization blueprint -- VULNERABLE variant.

GET /strings uses the Accept-Language header to load a locale file.
The header value is used directly as the locale identifier without
validation against an allowlist.  An attacker can send an Accept-Language
value like ../../etc/passwd or ../../app/config to read arbitrary files
from the server.

Chain: Accept-Language header value -> locale file path -> open() reads sensitive file
Individual findings: unvalidated header value used in file path construction (medium)
Chain finding: locale file traversal reads application secrets via crafted header (high)
CWE-22: Improper Limitation of a Pathname to a Restricted Directory
"""
import os
from flask import Blueprint, request, jsonify
import config

i18n_bp = Blueprint("i18n", __name__)


@i18n_bp.route("/strings")
def get_strings():
    """Return locale strings for the requested language.

    VULNERABLE: reads Accept-Language header and constructs a file path
    from it without validating against the allowed locales list.
    """
    locale = request.headers.get("Accept-Language", "en").split(",")[0].strip().split(";")[0].strip()
    locale_file = os.path.join(config.LOCALES_DIR, locale + ".json")
# vuln-code-snippet start chain_locale_traversal_vuln
    with open(locale_file, "r") as f:  # vuln-code-snippet vuln-line chain_locale_traversal_vuln
        strings = f.read()
# vuln-code-snippet end chain_locale_traversal_vuln
    return jsonify({"locale": locale, "strings": strings})

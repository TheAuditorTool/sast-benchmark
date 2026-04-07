import os
from flask import Blueprint, request, jsonify
import config

i18n_bp = Blueprint("i18n", __name__)

@i18n_bp.route("/strings")
def get_strings():
    raw = request.headers.get("Accept-Language", "en").split(",")[0].strip().split(";")[0].strip()
    locale = raw if raw in config.ALLOWED_LOCALES else "en"
    locale_file = os.path.join(config.LOCALES_DIR, locale + ".json")
# vuln-code-snippet start ChainScenario0004A
    with open(locale_file, "r") as f:  # vuln-code-snippet target-line ChainScenario0004A
        strings = f.read()
# vuln-code-snippet end ChainScenario0004A
    return jsonify({"locale": locale, "strings": strings})

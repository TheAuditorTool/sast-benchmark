import os
from flask import Blueprint, request, jsonify
import module_b

i18n_bp = Blueprint("i18n", __name__)

@i18n_bp.route("/strings")
def get_strings():
    locale = request.headers.get("Accept-Language", "en").split(",")[0].strip().split(";")[0].strip()
    locale_file = os.path.join(config.LOCALES_DIR, locale + ".json")
    with open(locale_file, "r") as f:
        strings = f.read()
    return jsonify({"locale": locale, "strings": strings})

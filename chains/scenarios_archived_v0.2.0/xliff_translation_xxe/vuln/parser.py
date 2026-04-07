"""XLIFF translation file parser -- VULNERABLE variant.

POST /translations/import saves an XLIFF file and parses it with
xml.etree.ElementTree's default parser. XLIFF is a standard XML-based
translation exchange format. An attacker can submit an XLIFF file with
an external entity pointing to /run/secrets/stripe_key or similar:

    <!DOCTYPE xliff [<!ENTITY apikey SYSTEM "file:///run/secrets/stripe_key">]>
    <xliff version="1.2">
      <file><body><trans-unit id="1">
        <source>&apikey;</source>
      </trans-unit></body></file>
    </xliff>

The external entity is resolved and the secret appears in the translation unit.

Chain: XLIFF upload -> ET.parse (entities enabled) -> API key file read
Individual findings: XXE via XLIFF translation import (high)
Chain finding: XXE enables reading API keys via translation file import (critical)
CWE-611: Improper Restriction of XML External Entity Reference
"""
import os
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify, current_app
from werkzeug.utils import secure_filename
import config

xliff_bp = Blueprint("xliff", __name__)

_XLIFF_NS = "urn:oasis:names:tc:xliff:document:1.2"


def _allowed(filename):
    return "." in filename and filename.rsplit(".", 1)[1].lower() in config.ALLOWED_EXTENSIONS


@xliff_bp.route("/translations/import", methods=["POST"])
def import_translations():
    """Accept an XLIFF file upload and return extracted translation units."""
    if "file" not in request.files:
        return jsonify({"error": "no file field"}), 400
    f = request.files["file"]
    if not f or not _allowed(f.filename):
        return jsonify({"error": "only XLIFF files accepted"}), 400
    filename = secure_filename(f.filename)
    save_path = os.path.join(current_app.config["UPLOAD_FOLDER"], filename)
    f.save(save_path)
    try:
# vuln-code-snippet start chain_xliff_xxe_vuln
        tree = ET.parse(save_path)  # vuln-code-snippet vuln-line chain_xliff_xxe_vuln
# vuln-code-snippet end chain_xliff_xxe_vuln
        root = tree.getroot()
        units = []
        for tu in root.iter("trans-unit"):
            src = tu.findtext("source") or tu.findtext(f"{{{_XLIFF_NS}}}source") or ""
            tgt = tu.findtext("target") or tu.findtext(f"{{{_XLIFF_NS}}}target") or ""
            units.append({"id": tu.get("id", ""), "source": src, "target": tgt})
        return jsonify({"units": units})
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    finally:
        if os.path.exists(save_path):
            os.remove(save_path)

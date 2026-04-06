"""XLIFF translation file parser -- SAFE variant.

POST /translations/import saves an XLIFF file and parses it with a
hardened XMLParser. Clearing parser.entity prevents any external entity
declaration from being resolved, so crafted XLIFF files that target
/run/secrets/stripe_key or other sensitive paths produce no file read.

Chain broken: XMLParser with entity={} blocks external entity resolution ->
API key exfiltration via XLIFF is prevented
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


def _safe_parse(path):
    parser = ET.XMLParser()
    parser.entity = {}
    return ET.parse(path, parser=parser)


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
# vuln-code-snippet start chain_xliff_xxe_safe
        tree = _safe_parse(save_path)  # vuln-code-snippet safe-line chain_xliff_xxe_safe
# vuln-code-snippet end chain_xliff_xxe_safe
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

"""SVG upload handler -- VULNERABLE variant.

POST /avatar saves the uploaded SVG and parses it as XML to extract
metadata (title, description). Because SVG files are XML documents,
an attacker can craft an SVG with an external entity:

    <!DOCTYPE svg [<!ENTITY secret SYSTEM "file:///etc/passwd">]>
    <svg xmlns="http://www.w3.org/2000/svg">
      <title>&secret;</title>
    </svg>

The default ET.parse call resolves the entity and embeds the file
contents in the returned metadata.

Chain: SVG file upload -> ET.parse (entities enabled) -> file read
Individual findings: XXE via SVG upload (high)
Chain finding: XXE enables reading /etc/passwd via SVG avatar upload (critical)
CWE-611: Improper Restriction of XML External Entity Reference
"""
import os
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify, current_app
from werkzeug.utils import secure_filename
import config

svg_bp = Blueprint("svg", __name__)

_SVG_NS = "http://www.w3.org/2000/svg"


def _allowed(filename):
    return "." in filename and filename.rsplit(".", 1)[1].lower() in config.ALLOWED_EXTENSIONS


@svg_bp.route("/avatar", methods=["POST"])
def upload_avatar():
    """Save an SVG avatar and return extracted title/description metadata."""
    if "file" not in request.files:
        return jsonify({"error": "no file field"}), 400
    f = request.files["file"]
    if not f or not _allowed(f.filename):
        return jsonify({"error": "only SVG files accepted"}), 400
    filename = secure_filename(f.filename)
    save_path = os.path.join(current_app.config["UPLOAD_FOLDER"], filename)
    f.save(save_path)
    try:
# vuln-code-snippet start chain_svg_xxe_vuln
        tree = ET.parse(save_path)  # vuln-code-snippet vuln-line chain_svg_xxe_vuln
# vuln-code-snippet end chain_svg_xxe_vuln
        root = tree.getroot()
        title = root.findtext(f"{{{_SVG_NS}}}title") or root.findtext("title") or ""
        desc = root.findtext(f"{{{_SVG_NS}}}desc") or root.findtext("desc") or ""
        return jsonify({"title": title, "description": desc})
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    finally:
        if os.path.exists(save_path):
            os.remove(save_path)

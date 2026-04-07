"""DOCX document parser -- VULNERABLE variant.

POST /document/upload accepts a DOCX file, unzips it in memory, and
parses word/document.xml with xml.etree.ElementTree's default parser.
A specially crafted document.xml containing an external entity pointing
to file:///etc/passwd will have that entity resolved during parsing and
the file contents appear in the extracted paragraph text.

Chain: DOCX upload -> ZIP extraction -> ET.parse (entities enabled) -> file read
Individual findings: XXE via DOCX document upload (high)
Chain finding: XXE enables reading server files via DOCX upload (critical)
CWE-611: Improper Restriction of XML External Entity Reference
"""
import io
import zipfile
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify
from werkzeug.utils import secure_filename
import config

docx_bp = Blueprint("docx", __name__)

_W_NS = "http://schemas.openxmlformats.org/wordprocessingml/2006/main"


def _allowed(filename):
    return "." in filename and filename.rsplit(".", 1)[1].lower() in config.ALLOWED_EXTENSIONS


@docx_bp.route("/document/upload", methods=["POST"])
def upload_docx():
    """Accept a DOCX upload and return extracted paragraph text."""
    if "file" not in request.files:
        return jsonify({"error": "no file field"}), 400
    f = request.files["file"]
    if not f or not _allowed(f.filename):
        return jsonify({"error": "only DOCX files accepted"}), 400
    raw = f.read()
    if len(raw) > config.MAX_DOCX_BYTES:
        return jsonify({"error": "file too large"}), 413
    try:
        zf = zipfile.ZipFile(io.BytesIO(raw))
        doc_xml = zf.read("word/document.xml")
    except (zipfile.BadZipFile, KeyError) as exc:
        return jsonify({"error": str(exc)}), 422
    try:
# vuln-code-snippet start chain_docx_xxe_vuln
        root = ET.fromstring(doc_xml)  # vuln-code-snippet vuln-line chain_docx_xxe_vuln
# vuln-code-snippet end chain_docx_xxe_vuln
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    paragraphs = [
        "".join(t.text or "" for t in para.iter(f"{{{_W_NS}}}t"))
        for para in root.iter(f"{{{_W_NS}}}p")
    ]
    return jsonify({"paragraphs": paragraphs})

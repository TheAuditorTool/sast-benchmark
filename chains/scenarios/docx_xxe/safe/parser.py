"""DOCX document parser -- SAFE variant.

POST /document/upload accepts a DOCX file, unzips it in memory, and
parses word/document.xml with a hardened XMLParser that has its entity
table cleared. External entity declarations inside the XML part are
therefore not resolved, preventing any local file from being read via
a crafted DOCX.

Chain broken: XMLParser with entity={} blocks external entity resolution ->
file read is prevented even when processing crafted DOCX archives
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


def _safe_fromstring(data):
    parser = ET.XMLParser()
    parser.entity = {}
    return ET.fromstring(data, parser=parser)


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
# vuln-code-snippet start chain_docx_xxe_safe
        root = _safe_fromstring(doc_xml)  # vuln-code-snippet safe-line chain_docx_xxe_safe
# vuln-code-snippet end chain_docx_xxe_safe
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    paragraphs = [
        "".join(t.text or "" for t in para.iter(f"{{{_W_NS}}}t"))
        for para in root.iter(f"{{{_W_NS}}}p")
    ]
    return jsonify({"paragraphs": paragraphs})

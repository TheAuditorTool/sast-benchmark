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
# vuln-code-snippet start ChainScenario0200A
        root = _safe_fromstring(doc_xml)  # vuln-code-snippet target-line ChainScenario0200A
# vuln-code-snippet end ChainScenario0200A
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    paragraphs = [
        "".join(t.text or "" for t in para.iter(f"{{{_W_NS}}}t"))
        for para in root.iter(f"{{{_W_NS}}}p")
    ]
    return jsonify({"paragraphs": paragraphs})

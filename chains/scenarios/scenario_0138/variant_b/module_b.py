import os
import xml.etree.ElementTree as ET
from flask import Blueprint, request, jsonify, current_app
from werkzeug.utils import secure_filename
import module_c as cfg

parser_bp = Blueprint("parser", __name__)

def _allowed(filename):
    return "." in filename and filename.rsplit(".", 1)[1].lower() in cfg.ALLOWED_EXTENSIONS

def _safe_parse(path):
    parser = ET.XMLParser()
    parser.entity = {}
    tree = ET.parse(path, parser=parser)
    return tree

@parser_bp.route("/upload", methods=["POST"])
def upload_xml():
    if "file" not in request.files:
        return jsonify({"error": "no file field"}), 400
    f = request.files["file"]
    if not f or not _allowed(f.filename):
        return jsonify({"error": "invalid file"}), 400
    filename = secure_filename(f.filename)
    save_path = os.path.join(current_app.config["UPLOAD_FOLDER"], filename)
    f.save(save_path)
    try:
        tree = _safe_parse(save_path)
        root = tree.getroot()
        data = {child.tag: child.text for child in root}
        return jsonify({"parsed": data})
    except ET.ParseError as exc:
        return jsonify({"error": str(exc)}), 422
    finally:
        if os.path.exists(save_path):
            os.remove(save_path)

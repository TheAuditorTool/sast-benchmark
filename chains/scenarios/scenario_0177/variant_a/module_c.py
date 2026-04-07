import os
import zipfile
from flask import Blueprint, request, jsonify

extractor_bp = Blueprint("extractor", __name__)

PLUGINS_DIR = os.environ.get("PLUGINS_DIR", "/var/app/plugins")
MAX_ARCHIVE_SIZE = 10 * 1024 * 1024  

def extract_archive(archive_path: str, dest_dir: str) -> list[str]:
    extracted = []
    with zipfile.ZipFile(archive_path, "r") as zf:
        for entry in zf.infolist():
            target = os.path.join(dest_dir, entry.filename)
            if entry.is_dir():
                os.makedirs(target, exist_ok=True)
            else:
                os.makedirs(os.path.dirname(target), exist_ok=True)
                with zf.open(entry) as src, open(target, "wb") as dst:
                    dst.write(src.read())
            extracted.append(target)
    return extracted

@extractor_bp.route("/plugins/upload", methods=["POST"])
def upload_plugin():
    if "bundle" not in request.files:
        return jsonify({"error": "No bundle file in request"}), 400

    bundle = request.files["bundle"]
    if bundle.filename == "":
        return jsonify({"error": "No filename provided"}), 400

    tmp_path = os.path.join("/tmp", bundle.filename)
    bundle.save(tmp_path)

    try:
        extracted = extract_archive(tmp_path, PLUGINS_DIR)
    except zipfile.BadZipFile:
        return jsonify({"error": "Invalid zip archive"}), 400
    finally:
        os.unlink(tmp_path)

    return jsonify({
        "status": "extracted",
        "files": extracted,
        "count": len(extracted),
    }), 201

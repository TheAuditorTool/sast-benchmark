"""Plugin zip extractor -- VULNERABLE variant.

Accepts an uploaded zip archive and extracts it into the plugins directory.
Entry filenames from the zip archive are used as-is when constructing the
destination path. An archive entry named ../../config/settings.json escapes
the plugins directory and overwrites the application config file read by
config_loader.py. This is the classic "zip slip" vulnerability.

Chain: POST /plugins/upload with malicious zip -> extract_archive() writes
       traversal entry -> CONFIG_PATH overwritten -> POST /admin/reload-config
       -> attacker-controlled config loaded
Individual findings: path traversal via zip entry names (high, CWE-434)
Chain finding: zip slip + config overwrite = application takeover (critical,
               CWE-434)
"""
import os
import zipfile
from flask import Blueprint, request, jsonify

extractor_bp = Blueprint("extractor", __name__)

PLUGINS_DIR = os.environ.get("PLUGINS_DIR", "/var/app/plugins")
MAX_ARCHIVE_SIZE = 10 * 1024 * 1024  # 10 MB


def extract_archive(archive_path: str, dest_dir: str) -> list[str]:
    """Extract a zip archive into dest_dir.

    Entry names are joined directly with dest_dir without validation,
    allowing ../ sequences to escape the target directory.
    """
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


# vuln-code-snippet start chain_zip_slip_vuln
@extractor_bp.route("/plugins/upload", methods=["POST"])
def upload_plugin():
    """Accept a plugin bundle zip archive and extract it.

    Expects multipart/form-data with a 'bundle' file field.
    """
    if "bundle" not in request.files:
        return jsonify({"error": "No bundle file in request"}), 400

    bundle = request.files["bundle"]
    if bundle.filename == "":
        return jsonify({"error": "No filename provided"}), 400

    tmp_path = os.path.join("/tmp", bundle.filename)
    bundle.save(tmp_path)

    try:
        extracted = extract_archive(tmp_path, PLUGINS_DIR)  # vuln-code-snippet vuln-line chain_zip_slip_vuln
    except zipfile.BadZipFile:
        return jsonify({"error": "Invalid zip archive"}), 400
    finally:
        os.unlink(tmp_path)

    return jsonify({
        "status": "extracted",
        "files": extracted,
        "count": len(extracted),
    }), 201
# vuln-code-snippet end chain_zip_slip_vuln

"""Plugin zip extractor -- SAFE variant.

Accepts an uploaded zip archive and extracts it into the plugins directory.
Each entry's resolved path is checked to confirm it remains inside PLUGINS_DIR
before any file is written. An entry named ../../config/settings.json is
detected and rejected before extraction, protecting the config file.

Chain: POST /plugins/upload with malicious zip -> extract_archive() resolves
       each entry path -> outside PLUGINS_DIR -> ValueError raised -> 400 returned
       -> no files written -> CONFIG_PATH intact
Individual findings: none -- path traversal blocked by validation
Chain finding: none -- zip slip prevented, config overwrite impossible (CWE-434 mitigated)
"""
import os
import zipfile
from flask import Blueprint, request, jsonify

extractor_bp = Blueprint("extractor", __name__)

PLUGINS_DIR = os.path.realpath(os.environ.get("PLUGINS_DIR", "/var/app/plugins"))
MAX_ARCHIVE_SIZE = 10 * 1024 * 1024  # 10 MB


def extract_archive(archive_path: str, dest_dir: str) -> list[str]:
    """Extract a zip archive into dest_dir.

    Each entry path is resolved and verified to remain within dest_dir.
    Raises ValueError if any entry would escape the destination.
    """
    real_dest = os.path.realpath(dest_dir)
    extracted = []
    with zipfile.ZipFile(archive_path, "r") as zf:
        for entry in zf.infolist():
            target = os.path.realpath(os.path.join(real_dest, entry.filename))
            if not target.startswith(real_dest + os.sep):
                raise ValueError(f"Zip entry escapes destination: {entry.filename!r}")
            if entry.is_dir():
                os.makedirs(target, exist_ok=True)
            else:
                os.makedirs(os.path.dirname(target), exist_ok=True)
                with zf.open(entry) as src, open(target, "wb") as dst:
                    dst.write(src.read())
            extracted.append(target)
    return extracted


# vuln-code-snippet start chain_zip_slip_safe
@extractor_bp.route("/plugins/upload", methods=["POST"])
def upload_plugin():
    """Accept a plugin bundle zip archive and extract it.

    Expects multipart/form-data with a 'bundle' file field.
    All zip entry paths are validated to stay within the plugins directory.
    """
    if "bundle" not in request.files:
        return jsonify({"error": "No bundle file in request"}), 400

    bundle = request.files["bundle"]
    if bundle.filename == "":
        return jsonify({"error": "No filename provided"}), 400

    tmp_path = os.path.join("/tmp", bundle.filename)
    bundle.save(tmp_path)

    try:
        extracted = extract_archive(tmp_path, PLUGINS_DIR)  # vuln-code-snippet safe-line chain_zip_slip_safe
    except zipfile.BadZipFile:
        return jsonify({"error": "Invalid zip archive"}), 400
    except ValueError as exc:
        return jsonify({"error": str(exc)}), 400
    finally:
        os.unlink(tmp_path)

    return jsonify({
        "status": "extracted",
        "files": extracted,
        "count": len(extracted),
    }), 201
# vuln-code-snippet end chain_zip_slip_safe

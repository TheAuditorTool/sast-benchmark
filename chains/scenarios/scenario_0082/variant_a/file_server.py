import os
from flask import Blueprint, send_from_directory, abort

file_server_bp = Blueprint("file_server", __name__)
REPORTS_DIR = os.path.join(os.path.dirname(__file__), "reports")

@file_server_bp.route("/files/<path:filename>")
def serve_file(filename):
    if not os.path.isdir(REPORTS_DIR):
        abort(404)
    safe_path = os.path.realpath(os.path.join(REPORTS_DIR, filename))
    if not safe_path.startswith(os.path.realpath(REPORTS_DIR)):
        abort(400)
    return send_from_directory(REPORTS_DIR, filename)

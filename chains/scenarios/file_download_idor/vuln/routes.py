"""Flask routes for the document download API -- VULNERABLE variant.

GET /api/documents/<file_id>/download serves the file referenced by
file_id without checking that the authenticated user owns that
document.  Document IDs are sequential integers so an attacker can
enumerate all uploaded files.

Chain: authenticated session + sequential file_id -> unowned file bytes
Individual findings: missing ownership check on download (medium)
Chain finding: bulk exfiltration of all user documents (high)
CWE-862: Missing Authorization
"""
import os
from flask import Flask, jsonify, send_file, session
from auth import login_required, get_current_user_id
from models import get_document_metadata, get_document_path, list_user_documents

app = Flask(__name__)
app.secret_key = "dev-secret-replace-in-prod"


@app.route("/api/session/login", methods=["POST"])
def login():
    from flask import request
    data = request.get_json(force=True) or {}
    user_id = data.get("user_id")
    if not user_id:
        return jsonify({"error": "user_id required"}), 400
    session["user_id"] = int(user_id)
    return jsonify({"ok": True})


@app.route("/api/documents")
@login_required
def list_documents():
    """Return document list for the authenticated user."""
    uid = get_current_user_id()
    return jsonify(list_user_documents(uid))


@app.route("/api/documents/<int:file_id>/download")
@login_required
def download_document(file_id):
    """Stream a document to the caller.

    VULNERABLE: no ownership check -- any authenticated user can
    download any document by incrementing file_id.
    """
# vuln-code-snippet start chain_file_download_idor_vuln
    meta = get_document_metadata(file_id)  # vuln-code-snippet vuln-line chain_file_download_idor_vuln
# vuln-code-snippet end chain_file_download_idor_vuln
    if meta is None:
        return jsonify({"error": "Document not found"}), 404
    path = get_document_path(file_id)
    if not path or not os.path.exists(path):
        return jsonify({"error": "File unavailable"}), 404
    return send_file(path, mimetype=meta["content_type"],
                     download_name=meta["filename"], as_attachment=True)


if __name__ == "__main__":
    app.run(port=5000)

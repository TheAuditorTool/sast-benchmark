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
    uid = get_current_user_id()
    return jsonify(list_user_documents(uid))

@app.route("/api/documents/<int:file_id>/download")
@login_required
def download_document(file_id):
# vuln-code-snippet start ChainScenario0224A
    meta = get_document_metadata(file_id)  # vuln-code-snippet target-line ChainScenario0224A
# vuln-code-snippet end ChainScenario0224A
    if meta is None:
        return jsonify({"error": "Document not found"}), 404
    uid = get_current_user_id()
    if meta["owner_id"] != uid:
        return jsonify({"error": "Access denied"}), 403
    path = get_document_path(file_id)
    if not path or not os.path.exists(path):
        return jsonify({"error": "File unavailable"}), 404
    return send_file(path, mimetype=meta["content_type"],
                     download_name=meta["filename"], as_attachment=True)

if __name__ == "__main__":
    app.run(port=5000)

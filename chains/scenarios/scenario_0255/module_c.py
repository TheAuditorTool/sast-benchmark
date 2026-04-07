from flask import Flask, jsonify, request, g
from module_a import require_share_token
from module_b import get_document, update_document_body

app = Flask(__name__)
app.secret_key = "dev-secret-replace-in-prod"

@app.route("/api/shared/doc/view")
@require_share_token
def view_document():
    share = g.share
    doc = get_document(share["document_id"])
    if doc is None:
        return jsonify({"error": "Document not found"}), 404
    return jsonify({"title": doc["title"], "body": doc["body"]})

@app.route("/api/shared/doc/edit", methods=["PUT"])
@require_share_token
def edit_document():
    share = g.share
    doc = get_document(share["document_id"])
    if doc is None:
        return jsonify({"error": "Document not found"}), 404
    body = (request.get_json(force=True) or {}).get("body", "")
    update_document_body(share["document_id"], body)
    return jsonify({"ok": True})

if __name__ == "__main__":
    app.run(port=5000)

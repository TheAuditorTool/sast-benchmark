from flask import Flask, jsonify, request, g
from auth import require_share_token
from models import get_document, update_document_body

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
# vuln-code-snippet start ChainScenario0134B
    doc = get_document(share["document_id"])  # vuln-code-snippet target-line ChainScenario0134B
# vuln-code-snippet end ChainScenario0134B
    if doc is None:
        return jsonify({"error": "Document not found"}), 404
    body = (request.get_json(force=True) or {}).get("body", "")
    update_document_body(share["document_id"], body)
    return jsonify({"ok": True})

if __name__ == "__main__":
    app.run(port=5000)

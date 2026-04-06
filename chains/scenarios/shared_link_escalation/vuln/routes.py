"""Flask routes for the shared-link document API -- VULNERABLE variant.

A view-only share link (access_level='view') also grants access to the
edit endpoint because the edit route only checks that a valid token
exists, not that the token's access_level permits write operations.

Chain: view-only token -> /api/shared/doc/edit endpoint -> document
  body overwritten or read at escalated privilege
Individual findings: missing access_level check on edit route (medium)
Chain finding: view-only recipients can modify documents (high)
CWE-862: Missing Authorization
"""
from flask import Flask, jsonify, request, g
from auth import require_share_token
from models import get_document, update_document_body

app = Flask(__name__)
app.secret_key = "dev-secret-replace-in-prod"


@app.route("/api/shared/doc/view")
@require_share_token
def view_document():
    """Return document content for read access."""
    share = g.share
    doc = get_document(share["document_id"])
    if doc is None:
        return jsonify({"error": "Document not found"}), 404
    return jsonify({"title": doc["title"], "body": doc["body"]})


@app.route("/api/shared/doc/edit", methods=["PUT"])
@require_share_token
def edit_document():
    """Update document body.

    VULNERABLE: only checks that a valid share token was supplied.
    A view-only token has access_level='view' but this route never
    inspects that field, so view recipients can overwrite the document.
    """
    share = g.share
# vuln-code-snippet start chain_shared_link_vuln
    doc = get_document(share["document_id"])  # vuln-code-snippet vuln-line chain_shared_link_vuln
# vuln-code-snippet end chain_shared_link_vuln
    if doc is None:
        return jsonify({"error": "Document not found"}), 404
    body = (request.get_json(force=True) or {}).get("body", "")
    update_document_body(share["document_id"], body)
    return jsonify({"ok": True})


if __name__ == "__main__":
    app.run(port=5000)

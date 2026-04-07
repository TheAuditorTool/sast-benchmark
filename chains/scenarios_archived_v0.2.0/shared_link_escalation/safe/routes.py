"""Flask routes for the shared-link document API -- SAFE variant.

The edit endpoint verifies that the share token's access_level is
'edit' before allowing any write operation.  View-only tokens receive
403 on the edit route.

Chain: broken -- access_level enforced on write route
CWE-862: Missing Authorization (remediated)
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

    FIXED: the route requires access_level='edit'.  View-only share
    tokens receive 403 before any data is accessed or modified.
    """
    share = g.share
    if share["access_level"] != "edit":
        return jsonify({"error": "Write access not granted by this token"}), 403

# vuln-code-snippet start chain_shared_link_safe
    doc = get_document(share["document_id"])  # vuln-code-snippet safe-line chain_shared_link_safe
# vuln-code-snippet end chain_shared_link_safe
    if doc is None:
        return jsonify({"error": "Document not found"}), 404
    body = (request.get_json(force=True) or {}).get("body", "")
    update_document_body(share["document_id"], body)
    return jsonify({"ok": True})


if __name__ == "__main__":
    app.run(port=5000)

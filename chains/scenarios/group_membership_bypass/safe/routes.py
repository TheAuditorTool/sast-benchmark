"""Document access routes -- SAFE variant.

GET /groups/<group_id>/documents uses check_group_access() which now
verifies that the caller is actually a member of the group.

Chain: user supplies grp-confidential -> membership verified -> 403 if not member
"""
from flask import request, jsonify
from models import app, GROUPS, DOCUMENTS
from auth import require_login, check_group_access


# vuln-code-snippet start chain_group_bypass_safe
@app.route("/groups/<group_id>/documents", methods=["GET"])
@require_login
def list_group_documents(group_id):
    """List documents for a group. SAFE: membership verified against server-side store."""
    user_id = request.headers.get("X-User-Id", "")
    if not check_group_access(user_id, group_id):
        return jsonify({"error": "Group not found or access denied"}), 403
    group = GROUPS[group_id]
    docs = {doc_id: DOCUMENTS[doc_id] for doc_id in group["documents"] if doc_id in DOCUMENTS}
    return jsonify({"group": group["name"], "documents": docs})  # vuln-code-snippet safe-line chain_group_bypass_safe
# vuln-code-snippet end chain_group_bypass_safe


@app.route("/health")
def health():
    """Health check."""
    return jsonify({"status": "ok"})


if __name__ == "__main__":
    app.run(port=5000)

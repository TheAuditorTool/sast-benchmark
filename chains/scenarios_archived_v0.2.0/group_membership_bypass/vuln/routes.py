"""Document access routes -- VULNERABLE variant.

GET /groups/<group_id>/documents uses check_group_access() which only
verifies the group exists, not that the caller is a member.

Chain: user supplies grp-confidential -> group exists check passes -> docs returned
"""
from flask import request, jsonify
from models import app, GROUPS, DOCUMENTS
from auth import require_login, check_group_access


# vuln-code-snippet start chain_group_bypass_vuln
@app.route("/groups/<group_id>/documents", methods=["GET"])
@require_login
def list_group_documents(group_id):
    """List documents for a group. VULNERABLE: membership not verified."""
    user_id = request.headers.get("X-User-Id", "")
    if not check_group_access(user_id, group_id):
        return jsonify({"error": "Group not found or access denied"}), 403
    group = GROUPS[group_id]
    docs = {doc_id: DOCUMENTS[doc_id] for doc_id in group["documents"] if doc_id in DOCUMENTS}
    return jsonify({"group": group["name"], "documents": docs})  # vuln-code-snippet vuln-line chain_group_bypass_vuln
# vuln-code-snippet end chain_group_bypass_vuln


@app.route("/health")
def health():
    """Health check."""
    return jsonify({"status": "ok"})


if __name__ == "__main__":
    app.run(port=5000)

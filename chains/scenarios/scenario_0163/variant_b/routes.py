from flask import request, jsonify
from models import app, GROUPS, DOCUMENTS
from auth import require_login, check_group_access

# vuln-code-snippet start ChainScenario0163B
@app.route("/groups/<group_id>/documents", methods=["GET"])
@require_login
def list_group_documents(group_id):
    user_id = request.headers.get("X-User-Id", "")
    if not check_group_access(user_id, group_id):
        return jsonify({"error": "Group not found or access denied"}), 403
    group = GROUPS[group_id]
    docs = {doc_id: DOCUMENTS[doc_id] for doc_id in group["documents"] if doc_id in DOCUMENTS}
    return jsonify({"group": group["name"], "documents": docs})  # vuln-code-snippet target-line ChainScenario0163B
# vuln-code-snippet end ChainScenario0163B

@app.route("/health")
def health():
    return jsonify({"status": "ok"})

if __name__ == "__main__":
    app.run(port=5000)

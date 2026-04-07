"""Flask routes for the GraphQL node-query API -- SAFE variant.

The node resolver checks object ownership before returning any data.
Objects owned by other users return a null node response.

Chain: broken -- ownership verified before object is returned
CWE-862: Missing Authorization (remediated)
"""
from flask import Flask, jsonify, request, session
from auth import login_required, get_current_user_id
from models import decode_node_id, fetch_node, get_owner_for_node

app = Flask(__name__)
app.secret_key = "dev-secret-replace-in-prod"


@app.route("/api/session/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    user_id = data.get("user_id")
    if not user_id:
        return jsonify({"error": "user_id required"}), 400
    session["user_id"] = int(user_id)
    return jsonify({"ok": True})


@app.route("/api/graphql", methods=["POST"])
@login_required
def graphql_endpoint():
    """Execute a minimal GraphQL node query.

    FIXED: ownership is checked before returning the node.  Objects
    owned by other users produce a null data response (same as
    not-found so as not to confirm existence).
    """
    body = request.get_json(force=True) or {}
    node_id = body.get("id")
    if not node_id:
        return jsonify({"error": "id required"}), 400

    type_name, pk = decode_node_id(node_id)
    if type_name is None:
        return jsonify({"error": "Invalid node id"}), 400

    uid = get_current_user_id()
    owner = get_owner_for_node(type_name, pk)
    if owner is not None and owner != uid:
        return jsonify({"data": {"node": None}})

# vuln-code-snippet start chain_graphql_node_safe
    node = fetch_node(type_name, pk)  # vuln-code-snippet safe-line chain_graphql_node_safe
# vuln-code-snippet end chain_graphql_node_safe

    if node is None:
        return jsonify({"data": None})
    return jsonify({"data": {"node": node}})


if __name__ == "__main__":
    app.run(port=5000)

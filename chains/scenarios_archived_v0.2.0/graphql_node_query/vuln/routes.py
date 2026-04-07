"""Flask routes for the GraphQL node-query API -- VULNERABLE variant.

POST /api/graphql handles a minimal GraphQL-like 'node(id)' query.
The resolver decodes the node id, fetches the object, and returns it
without checking whether the authenticated user owns the object.
Any base64-encoded node id returns the underlying data.

Chain: authenticated session + crafted node_id (base64 'Type:pk') ->
  any object of any type returned regardless of ownership
Individual findings: missing ACL in node resolver (medium)
Chain finding: cross-user object exfiltration across all types (high)
CWE-862: Missing Authorization
"""
from flask import Flask, jsonify, request, session
from auth import login_required, get_current_user_id
from models import decode_node_id, fetch_node

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

    VULNERABLE: the node resolver returns any object regardless of
    who owns it.  Attacker crafts base64('Order:1'), base64('Order:2')
    etc. to read all orders.
    """
    body = request.get_json(force=True) or {}
    node_id = body.get("id")
    if not node_id:
        return jsonify({"error": "id required"}), 400

    type_name, pk = decode_node_id(node_id)
    if type_name is None:
        return jsonify({"error": "Invalid node id"}), 400

# vuln-code-snippet start chain_graphql_node_vuln
    node = fetch_node(type_name, pk)  # vuln-code-snippet vuln-line chain_graphql_node_vuln
# vuln-code-snippet end chain_graphql_node_vuln

    if node is None:
        return jsonify({"data": None})
    return jsonify({"data": {"node": node}})


if __name__ == "__main__":
    app.run(port=5000)

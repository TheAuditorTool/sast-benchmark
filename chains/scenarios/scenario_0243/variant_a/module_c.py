from flask import Flask, jsonify, request, session
from module_a import login_required, get_current_user_id
from module_b import decode_node_id, fetch_node, get_owner_for_node

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

    node = fetch_node(type_name, pk)

    if node is None:
        return jsonify({"data": None})
    return jsonify({"data": {"node": node}})

if __name__ == "__main__":
    app.run(port=5000)

import functools
from flask import request, jsonify
from module_a import app, USERS
from module_c import deserialize_mutation_input

def _require_auth(f):
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        if request.headers.get("X-User-Id") not in USERS:
            return jsonify({"error": "Authentication required"}), 401
        return f(*args, **kwargs)
    return decorated

@app.route("/graphql", methods=["POST"])
@_require_auth
def graphql_endpoint():
    caller_id = request.headers.get("X-User-Id")
    body = request.get_json(force=True) or {}
    operation = body.get("operationName", "")
    variables = body.get("variables", {})

    if operation == "updateUser":
        target_id = variables.get("userId", caller_id)
        if target_id != caller_id:
            return jsonify({"errors": [{"message": "Forbidden"}]}), 403
        if target_id not in USERS:
            return jsonify({"errors": [{"message": "Not found"}]}), 404
        input_obj = variables.get("input", {})
        updates = deserialize_mutation_input(input_obj)
        USERS[target_id].update(updates)
        return jsonify({"data": {"updateUser": USERS[target_id]}})

    return jsonify({"errors": [{"message": "Unknown operation"}]}), 400

if __name__ == "__main__":
    app.run(port=5000)

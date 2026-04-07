from flask import request, jsonify
from module_a import app
from module_b import resolve_update_project, resolve_add_member

@app.route("/graphql", methods=["POST"])
def graphql_endpoint():
    payload = request.get_json(force=True) or {}
    mutation = payload.get("mutation", {})
    response = {}

    if "updateProject" in mutation:
        args = mutation["updateProject"]
        result, status = resolve_update_project(
            project_id=args.get("project_id"),
            name=args.get("name"),
        )
        response["updateProject"] = result
        if status != 200:
            return jsonify(response), status

    if "addMember" in mutation:
        args = mutation["addMember"]
        result, status = resolve_add_member(
            project_id=args.get("project_id"),
            user_id=args.get("user_id"),
        )
        response["addMember"] = result

    return jsonify(response)

if __name__ == "__main__":
    app.run(port=5000)

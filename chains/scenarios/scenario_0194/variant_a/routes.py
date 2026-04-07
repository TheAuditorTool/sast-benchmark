from flask import request, jsonify
from models import app
from resolvers import resolve_update_project, resolve_add_member

# vuln-code-snippet start ChainScenario0194A
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
        result, status = resolve_add_member(  # vuln-code-snippet target-line ChainScenario0194A
            project_id=args.get("project_id"),
            user_id=args.get("user_id"),
        )
        response["addMember"] = result

    return jsonify(response)
# vuln-code-snippet end ChainScenario0194A

if __name__ == "__main__":
    app.run(port=5000)

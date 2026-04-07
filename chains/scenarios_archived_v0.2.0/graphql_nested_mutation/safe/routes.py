"""GraphQL mutation endpoint -- SAFE variant.

POST /graphql accepts a mutation dict. The nested addMember resolver now
independently verifies the caller's authorization for the target project.

Chain: member of proj-1 -> nested addMember(proj-2) -> resolver checks proj-2 membership -> 403
"""
from flask import request, jsonify
from models import app
from resolvers import resolve_update_project, resolve_add_member


# vuln-code-snippet start chain_graphql_nested_safe
@app.route("/graphql", methods=["POST"])
def graphql_endpoint():
    """Dispatch GraphQL-style mutations. SAFE: each resolver enforces its own authorization."""
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
        result, status = resolve_add_member(  # vuln-code-snippet safe-line chain_graphql_nested_safe
            project_id=args.get("project_id"),
            user_id=args.get("user_id"),
        )
        response["addMember"] = result

    return jsonify(response)
# vuln-code-snippet end chain_graphql_nested_safe


if __name__ == "__main__":
    app.run(port=5000)

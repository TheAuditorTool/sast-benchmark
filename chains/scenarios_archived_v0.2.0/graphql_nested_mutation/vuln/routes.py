"""GraphQL mutation endpoint -- VULNERABLE variant.

POST /graphql accepts a mutation dict. If the payload contains both
updateProject and a nested addMember operation, the nested resolver
executes without per-operation authorization.

Chain: member of proj-1 -> updateProject(proj-1) authorized -> nested addMember(proj-2) executes
"""
from flask import request, jsonify
from models import app
from resolvers import resolve_update_project, resolve_add_member


# vuln-code-snippet start chain_graphql_nested_vuln
@app.route("/graphql", methods=["POST"])
def graphql_endpoint():
    """Dispatch GraphQL-style mutations. VULNERABLE: nested resolver auth not enforced."""
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
        result, status = resolve_add_member(  # vuln-code-snippet vuln-line chain_graphql_nested_vuln
            project_id=args.get("project_id"),
            user_id=args.get("user_id"),
        )
        response["addMember"] = result

    return jsonify(response)
# vuln-code-snippet end chain_graphql_nested_vuln


if __name__ == "__main__":
    app.run(port=5000)

"""Admin project management routes -- SAFE variant.

The /admin/projects/archive endpoint is now protected by require_admin,
which verifies the caller holds the admin role before proceeding.

Chain: authenticated user -> role check enforced -> 403 (chain broken)
"""
from flask import request, jsonify
from models import app, PROJECTS
from middleware import require_admin


@app.route("/api/projects", methods=["GET"])
def list_projects():
    """List all projects. Available to all authenticated users."""
    return jsonify({"projects": list(PROJECTS.values())})


# vuln-code-snippet start chain_user_admin_api_safe
@app.route("/admin/projects/archive", methods=["POST"])
@require_admin
def archive_project():
    """Archive a project. Admin-only, enforced by require_admin decorator."""
    project_id = request.json.get("project_id", "")
    project = PROJECTS.get(project_id)
    if project is None:
        return jsonify({"error": "Project not found"}), 404
    project["archived"] = True  # vuln-code-snippet safe-line chain_user_admin_api_safe
    return jsonify({"status": "archived", "project": project})
# vuln-code-snippet end chain_user_admin_api_safe


if __name__ == "__main__":
    app.run(port=5000)

"""Admin project management routes -- VULNERABLE variant.

The /admin/projects/archive endpoint is protected only by require_login,
not require_admin. Any authenticated user can archive any project.

Chain: authenticated user -> no role check -> privileged archive action
"""
from flask import request, jsonify
from models import app, PROJECTS
from middleware import require_login


@app.route("/api/projects", methods=["GET"])
@require_login
def list_projects():
    """List all projects. Available to all authenticated users."""
    return jsonify({"projects": list(PROJECTS.values())})


# vuln-code-snippet start chain_user_admin_api_vuln
@app.route("/admin/projects/archive", methods=["POST"])
@require_login
def archive_project():
    """Archive a project. Should be admin-only but missing role enforcement."""
    project_id = request.json.get("project_id", "")
    project = PROJECTS.get(project_id)
    if project is None:
        return jsonify({"error": "Project not found"}), 404
    project["archived"] = True  # vuln-code-snippet vuln-line chain_user_admin_api_vuln
    return jsonify({"status": "archived", "project": project})
# vuln-code-snippet end chain_user_admin_api_vuln


if __name__ == "__main__":
    app.run(port=5000)

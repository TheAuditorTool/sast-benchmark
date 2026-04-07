from flask import request, jsonify
from models import app, PROJECTS
from middleware import require_admin

@app.route("/api/projects", methods=["GET"])
def list_projects():
    return jsonify({"projects": list(PROJECTS.values())})

# vuln-code-snippet start ChainScenario0034B
@app.route("/admin/projects/archive", methods=["POST"])
@require_admin
def archive_project():
    project_id = request.json.get("project_id", "")
    project = PROJECTS.get(project_id)
    if project is None:
        return jsonify({"error": "Project not found"}), 404
    project["archived"] = True  # vuln-code-snippet target-line ChainScenario0034B
    return jsonify({"status": "archived", "project": project})
# vuln-code-snippet end ChainScenario0034B

if __name__ == "__main__":
    app.run(port=5000)

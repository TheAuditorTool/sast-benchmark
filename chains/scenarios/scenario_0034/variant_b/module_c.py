from flask import request, jsonify
from module_b import app, PROJECTS
from module_a import require_admin

@app.route("/api/projects", methods=["GET"])
def list_projects():
    return jsonify({"projects": list(PROJECTS.values())})

@app.route("/admin/projects/archive", methods=["POST"])
@require_admin
def archive_project():
    project_id = request.json.get("project_id", "")
    project = PROJECTS.get(project_id)
    if project is None:
        return jsonify({"error": "Project not found"}), 404
    project["archived"] = True
    return jsonify({"status": "archived", "project": project})

if __name__ == "__main__":
    app.run(port=5000)

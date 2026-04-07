import functools
from flask import request, jsonify
from module_a import app, PROJECTS
from module_c import deserialize_project_settings

def _require_member(f):
    @functools.wraps(f)
    def decorated(project_id, *args, **kwargs):
        caller = request.headers.get("X-User-Id", "")
        project = PROJECTS.get(project_id)
        if project is None:
            return jsonify({"error": "Not found"}), 404
        if caller != project["owner_id"]:
            return jsonify({"error": "Forbidden"}), 403
        return f(project_id, *args, **kwargs)
    return decorated

@app.route("/projects/<project_id>/settings", methods=["PATCH"])
@_require_member
def update_project_settings(project_id):
    data = request.get_json(force=True) or {}
    updates = deserialize_project_settings(data)
    PROJECTS[project_id].update(updates)
    return jsonify({"status": "updated", "project": PROJECTS[project_id]})

if __name__ == "__main__":
    app.run(port=5000)

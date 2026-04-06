"""Project settings routes.

This file is IDENTICAL between vuln/ and safe/ variants.

Chain:
  1. PATCH /projects/<id>/settings passes the full body to deserialize_project_settings.
  2. The returned dict may contain 'owner_id' set to the attacker's user ID.
  3. The project record is updated, transferring ownership.

CWE-915: Mass assignment of owner_id enables unauthorised ownership transfer.
"""
import functools
from flask import request, jsonify
from models import app, PROJECTS
from serializers import deserialize_project_settings


def _require_member(f):
    """Require X-User-Id to be the current owner or a named contributor."""
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
    """Update project settings."""
    data = request.get_json(force=True) or {}
    updates = deserialize_project_settings(data)
    PROJECTS[project_id].update(updates)
    return jsonify({"status": "updated", "project": PROJECTS[project_id]})


if __name__ == "__main__":
    app.run(port=5000)

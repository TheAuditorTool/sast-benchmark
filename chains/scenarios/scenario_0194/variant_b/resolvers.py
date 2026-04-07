from flask import request
from models import USERS, PROJECTS

def get_current_user():
    return request.headers.get("X-User-Id", ""), USERS.get(request.headers.get("X-User-Id", ""))

def resolve_update_project(project_id, name=None):
    caller_id, caller = get_current_user()
    if caller is None:
        return {"error": "Authentication required"}, 401
    project = PROJECTS.get(project_id)
    if project is None:
        return {"error": "Project not found"}, 404
    if caller_id not in project["members"] and caller.get("role") != "admin":
        return {"error": "Not a project member"}, 403
    if name:
        project["name"] = name
    return {"project": project}, 200

def resolve_add_member(project_id, user_id):
    caller_id, caller = get_current_user()
    if caller is None:
        return {"error": "Authentication required"}, 401
    project = PROJECTS.get(project_id)
    if project is None:
        return {"error": "Project not found"}, 404
    if caller_id not in project["members"] and caller.get("role") != "admin":
        return {"error": "Not authorized to add members to this project"}, 403
    if user_id not in USERS:
        return {"error": "User not found"}, 404
    if user_id not in project["members"]:
        project["members"].append(user_id)
    return {"project": project}, 200

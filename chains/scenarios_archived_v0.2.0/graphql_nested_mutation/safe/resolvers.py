"""GraphQL resolver layer -- SAFE variant.

resolve_add_member() now verifies that the caller is a member of the
target project (or is an admin) before adding a new member. The nested
resolver enforces its own authorization independently of the parent
mutation that triggered it.

Chain: member of proj-1 -> nested addMember(proj-2) -> caller not member of proj-2 -> 403
"""
from flask import request
from models import USERS, PROJECTS


def get_current_user():
    """Resolve caller from X-User-Id header."""
    return request.headers.get("X-User-Id", ""), USERS.get(request.headers.get("X-User-Id", ""))


def resolve_update_project(project_id, name=None):
    """Top-level resolver: update a project's name. Checks membership."""
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
    """Nested resolver: add a member to a project.

    SAFE: independently verifies that the caller is authorized for the
    target project_id before adding a member, regardless of which outer
    mutation triggered this resolver.
    """
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

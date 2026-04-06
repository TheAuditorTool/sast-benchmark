"""GraphQL resolver layer -- VULNERABLE variant.

The top-level updateProject resolver checks that the caller is a project
member before allowing edits. However, the nested addMember resolver
within the same mutation payload is executed without re-checking the
caller's authorization for the addMember operation specifically.

A member of project A can nest an addMember call targeting project B
inside a valid updateProject mutation for project A, and the member
addition in project B will succeed without authorization.

Chain: authorized updateProject -> nested addMember skips auth -> member added to unrelated project
Individual findings: missing authorization check (CWE-862)
Chain finding: nested GraphQL mutation bypasses parent resolver authorization
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

    VULNERABLE: does not re-verify that the caller is authorized for THIS
    project_id. The caller may be a member of a different project and
    chain this resolver inside a valid outer mutation.
    """
    project = PROJECTS.get(project_id)
    if project is None:
        return {"error": "Project not found"}, 404
    if user_id not in USERS:
        return {"error": "User not found"}, 404
    if user_id not in project["members"]:
        project["members"].append(user_id)
    return {"project": project}, 200

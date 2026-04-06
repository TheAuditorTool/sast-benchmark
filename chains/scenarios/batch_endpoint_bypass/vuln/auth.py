"""Task access authorization -- VULNERABLE variant.

require_login confirms authentication. check_task_ownership() is available
to verify per-item ownership, but the batch route only calls require_login
and then updates all supplied task IDs without a per-item owner check.

Chain: user -> batch endpoint skips per-item auth -> modifies tasks they don't own
Individual findings: missing authorization check (CWE-862)
Chain finding: batch endpoint authorization bypass to update arbitrary tasks
"""
import functools
from flask import request, jsonify
from models import USERS, TASKS


def get_current_user():
    """Resolve caller from X-User-Id header."""
    return USERS.get(request.headers.get("X-User-Id", ""))


def require_login(f):
    """Reject unauthenticated requests."""
    @functools.wraps(f)
    def decorated(*args, **kwargs):
        user = get_current_user()
        if user is None:
            return jsonify({"error": "Authentication required"}), 401
        request.current_user = user
        request.current_user_id = request.headers.get("X-User-Id", "")
        return f(*args, **kwargs)
    return decorated


def check_task_ownership(user_id, task_id):
    """Return True if user_id owns the task or is an admin."""
    user = USERS.get(user_id)
    if user and user.get("role") == "admin":
        return True
    task = TASKS.get(task_id)
    if task is None:
        return False
    return task.get("owner_id") == user_id

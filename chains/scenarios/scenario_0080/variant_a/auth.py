import functools
from flask import request, jsonify
from models import USERS, TASKS

def get_current_user():
    return USERS.get(request.headers.get("X-User-Id", ""))

def require_login(f):
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
    user = USERS.get(user_id)
    if user and user.get("role") == "admin":
        return True
    task = TASKS.get(task_id)
    if task is None:
        return False
    return task.get("owner_id") == user_id

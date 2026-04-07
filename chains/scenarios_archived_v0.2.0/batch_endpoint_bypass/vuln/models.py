"""Data models for the task management and batch processing service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "taskman-secret"

# user_id -> {username, role, owned_tasks}
USERS = {
    "u1": {"username": "noah", "role": "user", "owned_tasks": ["task-101", "task-102"]},
    "u2": {"username": "olivia", "role": "user", "owned_tasks": ["task-103"]},
    "a1": {"username": "admin", "role": "admin", "owned_tasks": []},
}

# task_id -> {title, owner_id, status}
TASKS = {
    "task-101": {"title": "Fix login bug", "owner_id": "u1", "status": "open"},
    "task-102": {"title": "Write tests", "owner_id": "u1", "status": "open"},
    "task-103": {"title": "Deploy release", "owner_id": "u2", "status": "open"},
    "task-104": {"title": "Review budget", "owner_id": "a1", "status": "open"},
}

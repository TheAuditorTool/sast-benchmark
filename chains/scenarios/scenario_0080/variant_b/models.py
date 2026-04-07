from flask import Flask

app = Flask(__name__)
app.secret_key = "taskman-secret"

USERS = {
    "u1": {"username": "noah", "role": "user", "owned_tasks": ["task-101", "task-102"]},
    "u2": {"username": "olivia", "role": "user", "owned_tasks": ["task-103"]},
    "a1": {"username": "admin", "role": "admin", "owned_tasks": []},
}

TASKS = {
    "task-101": {"title": "Fix login bug", "owner_id": "u1", "status": "open"},
    "task-102": {"title": "Write tests", "owner_id": "u1", "status": "open"},
    "task-103": {"title": "Deploy release", "owner_id": "u2", "status": "open"},
    "task-104": {"title": "Review budget", "owner_id": "a1", "status": "open"},
}

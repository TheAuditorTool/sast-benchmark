from flask import Flask

app = Flask(__name__)
app.secret_key = "project-secret"

PROJECTS = {
    "p1": {"name": "Alpha", "owner_id": "u1", "description": "First project", "public": False},
    "p2": {"name": "Beta",  "owner_id": "u2", "description": "Second project", "public": True},
}

"""Data models for the project settings service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "project-secret"

# project_id -> {name, owner_id, description, public}
PROJECTS = {
    "p1": {"name": "Alpha", "owner_id": "u1", "description": "First project", "public": False},
    "p2": {"name": "Beta",  "owner_id": "u2", "description": "Second project", "public": True},
}

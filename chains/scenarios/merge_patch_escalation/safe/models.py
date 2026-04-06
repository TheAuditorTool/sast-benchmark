"""Data models for the JSON Merge Patch access service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "merge-secret"

# resource_id -> {name, owner_id, access_level, content}
RESOURCES = {
    "r1": {"name": "Report Q1", "owner_id": "u1", "access_level": "private", "content": "..."},
    "r2": {"name": "Blog Post", "owner_id": "u2", "access_level": "public",  "content": "..."},
}

ACCESS_LEVELS = {"private", "internal", "public"}

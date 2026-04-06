"""Data models for the content management system.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "cms-secret"

# user_id -> {name, role}  -- roles: viewer, editor, admin
USERS = {
    "u1": {"name": "carol", "role": "viewer"},
    "u2": {"name": "dave", "role": "editor"},
    "a1": {"name": "superuser", "role": "admin"},
}

# article_id -> {title, body, published}
ARTICLES = {
    "art1": {"title": "Intro", "body": "Hello world", "published": False},
    "art2": {"title": "Guide", "body": "Step by step", "published": True},
}

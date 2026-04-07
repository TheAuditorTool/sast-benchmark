from flask import Flask

app = Flask(__name__)
app.secret_key = "cms-secret"

USERS = {
    "u1": {"name": "carol", "role": "viewer"},
    "u2": {"name": "dave", "role": "editor"},
    "a1": {"name": "superuser", "role": "admin"},
}

ARTICLES = {
    "art1": {"title": "Intro", "body": "Hello world", "published": False},
    "art2": {"title": "Guide", "body": "Step by step", "published": True},
}

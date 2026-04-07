from flask import Flask

app = Flask(__name__)
app.secret_key = "pm-service-secret"

USERS = {
    "u1": {"username": "alice", "role": "user"},
    "u2": {"username": "bob", "role": "user"},
    "a1": {"username": "admin", "role": "admin"},
}

PROJECTS = {
    "p1": {"name": "Alpha", "owner": "u1", "archived": False},
    "p2": {"name": "Beta", "owner": "u2", "archived": False},
}

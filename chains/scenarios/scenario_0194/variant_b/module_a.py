from flask import Flask

app = Flask(__name__)
app.secret_key = "graphql-svc-secret"

USERS = {
    "u1": {"username": "pete", "role": "member"},
    "u2": {"username": "quinn", "role": "member"},
    "a1": {"username": "root", "role": "admin"},
}

PROJECTS = {
    "proj-1": {"name": "Atlas", "members": ["u1", "a1"]},
    "proj-2": {"name": "Beacon", "members": ["u2", "a1"]},
}

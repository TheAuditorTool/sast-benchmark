from flask import Flask

app = Flask(__name__)
app.secret_key = "gql-introspect-secret"

USERS = {
    "u1": {"username": "alice", "role": "user",  "reset_token": None},
    "u2": {"username": "bob",   "role": "user",  "reset_token": None},
    "a1": {"username": "root",  "role": "admin", "reset_token": "reset-root-abc123"},
}

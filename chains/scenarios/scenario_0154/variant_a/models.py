from flask import Flask

app = Flask(__name__)
app.secret_key = "nested-secret"

USERS = {
    "u1": {"username": "alice", "settings": {"theme": "dark",  "role": "user"}},
    "u2": {"username": "bob",   "settings": {"theme": "light", "role": "user"}},
}

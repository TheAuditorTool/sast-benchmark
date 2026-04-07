from flask import Flask

app = Flask(__name__)
app.secret_key = "profile-secret"

USERS = {
    "1": {"username": "alice", "email": "alice@example.com", "bio": "Hi", "is_admin": False},
    "2": {"username": "bob",   "email": "bob@example.com",   "bio": "Hey", "is_admin": False},
    "99": {"username": "root", "email": "root@example.com",  "bio": "",    "is_admin": True},
}

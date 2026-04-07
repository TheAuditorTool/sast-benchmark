from flask import Flask

app = Flask(__name__)
app.secret_key = "gql-secret"

USERS = {
    "u1": {"username": "alice", "email": "alice@example.com", "role": "user", "subscription": "free"},
    "u2": {"username": "bob",   "email": "bob@example.com",   "role": "user", "subscription": "pro"},
}

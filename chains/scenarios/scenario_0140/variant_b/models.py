from flask import Flask

app = Flask(__name__)
app.secret_key = "cicd-api-secret"

API_TOKENS = {
    "tok-read-only-aaa": {"user": "alice", "scopes": ["read"]},
    "tok-read-write-bbb": {"user": "bob", "scopes": ["read", "write"]},
    "tok-full-ccc": {"user": "carol", "scopes": ["read", "write", "deploy"]},
}

PIPELINES = {
    "pipe1": {"name": "main-ci", "config": "default.yml", "last_run": None},
}

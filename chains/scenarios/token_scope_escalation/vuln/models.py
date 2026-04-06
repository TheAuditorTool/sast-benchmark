"""Data models for the CI/CD pipeline API service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "cicd-api-secret"

# api_token -> {user, scopes}
# scopes is a list: "read", "write", "deploy"
API_TOKENS = {
    "tok-read-only-aaa": {"user": "alice", "scopes": ["read"]},
    "tok-read-write-bbb": {"user": "bob", "scopes": ["read", "write"]},
    "tok-full-ccc": {"user": "carol", "scopes": ["read", "write", "deploy"]},
}

# pipeline_id -> {name, config, last_run}
PIPELINES = {
    "pipe1": {"name": "main-ci", "config": "default.yml", "last_run": None},
}

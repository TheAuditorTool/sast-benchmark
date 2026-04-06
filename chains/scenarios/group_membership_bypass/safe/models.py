"""Data models for the document sharing and group management service.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)
app.secret_key = "docshare-secret"

# user_id -> {name, group_ids}
USERS = {
    "u1": {"name": "eve", "group_ids": ["grp-public"]},
    "u2": {"name": "frank", "group_ids": ["grp-public", "grp-confidential"]},
}

# group_id -> {name, documents}
GROUPS = {
    "grp-public": {"name": "Public", "documents": ["doc-open"]},
    "grp-confidential": {"name": "Confidential", "documents": ["doc-secret"]},
    "grp-admin": {"name": "Admin", "documents": ["doc-admin-only"]},
}

# document_id -> content
DOCUMENTS = {
    "doc-open": "This is a public document.",
    "doc-secret": "Confidential project data.",
    "doc-admin-only": "Admin-only system configuration.",
}

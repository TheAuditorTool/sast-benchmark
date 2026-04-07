from flask import Flask

app = Flask(__name__)
app.secret_key = "docshare-secret"

USERS = {
    "u1": {"name": "eve", "group_ids": ["grp-public"]},
    "u2": {"name": "frank", "group_ids": ["grp-public", "grp-confidential"]},
}

GROUPS = {
    "grp-public": {"name": "Public", "documents": ["doc-open"]},
    "grp-confidential": {"name": "Confidential", "documents": ["doc-secret"]},
    "grp-admin": {"name": "Admin", "documents": ["doc-admin-only"]},
}

DOCUMENTS = {
    "doc-open": "This is a public document.",
    "doc-secret": "Confidential project data.",
    "doc-admin-only": "Admin-only system configuration.",
}

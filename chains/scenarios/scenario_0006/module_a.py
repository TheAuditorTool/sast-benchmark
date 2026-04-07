from flask import Flask

app = Flask(__name__)
app.secret_key = "org-secret"

ORGS = {
    "org1": {"name": "Acme", "owner_id": "u1"},
}

MEMBERSHIPS = {}
_next_id = 1

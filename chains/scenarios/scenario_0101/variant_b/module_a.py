from flask import Flask

app = Flask(__name__)
app.secret_key = "billing-svc-secret"

VALID_ROLES = {"member", "billing_admin", "owner"}

MEMBERS = {}

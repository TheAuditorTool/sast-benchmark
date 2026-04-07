from flask import Flask

app = Flask(__name__)
app.secret_key = "bulk-secret"

USERS = {}

VALID_ROLES = {"user", "moderator", "admin"}
DEFAULT_ROLE = "user"

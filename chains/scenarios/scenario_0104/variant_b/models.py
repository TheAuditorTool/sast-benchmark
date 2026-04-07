from flask import Flask

app = Flask(__name__)
app.secret_key = "reg-secret"

USERS = {}

VALID_ROLES = {"user", "moderator", "admin"}

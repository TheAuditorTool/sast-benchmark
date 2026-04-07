from flask import Flask

app = Flask(__name__)
app.secret_key = "collab-secret-key"

INVITE_TOKENS = {"INV-ALPHA-001", "INV-BETA-002"}

USERS = {}

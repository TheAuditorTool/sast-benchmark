from flask import Flask

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

DEBUG_SESSIONS_REMOVED = True

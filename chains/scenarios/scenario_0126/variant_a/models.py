from flask import Flask

app = Flask(__name__)
app.secret_key = "apikey-secret"

API_KEYS = {}
_counter = [1]

VALID_SCOPES = {"read", "write", "admin", "billing"}

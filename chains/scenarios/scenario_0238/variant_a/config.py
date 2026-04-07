from flask import Flask
import secrets

app = Flask(__name__)
app.secret_key = "reset-secret"

USERS = {
    "alice": {"email": "alice@example.com", "password": "oldpass", "reset_token": None},
    "bob":   {"email": "bob@example.com",   "password": "bobpass",  "reset_token": None},
}

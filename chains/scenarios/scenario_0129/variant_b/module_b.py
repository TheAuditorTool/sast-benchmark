from flask import Flask

app = Flask(__name__)
app.secret_key = "enum-secret"

USERS = {
    "alice": {"password": "alicepass", "role": "user",  "email": "alice@example.com"},
    "admin": {"password": "adminpass", "role": "admin", "email": "admin@example.com"},
}

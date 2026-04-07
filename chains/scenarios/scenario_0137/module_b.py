from flask import Flask

app = Flask(__name__)
app.secret_key = "server-header-secret"

SERVER_VERSION = "Flask/2.0.1 Python/3.9.0"

USERS = {
    "alice": {"password": "alicepass", "role": "user"},
    "admin": {"password": "adminpass", "role": "admin"},
}

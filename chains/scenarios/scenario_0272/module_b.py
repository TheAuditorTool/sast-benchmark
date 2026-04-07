from flask import Flask

app = Flask(__name__)
app.secret_key = "swagger-secret"

ADMIN_TOKEN = "super-admin-token-xyz"

USERS = {
    "alice": {"password": "alicepass", "role": "user"},
    "admin": {"password": "adminpass", "role": "admin"},
}

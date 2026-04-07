from flask import Flask

app = Flask(__name__)
app.secret_key = "timing-secret"

USERS = {
    "alice": {"password": "correct-horse-battery-staple", "role": "user"},
    "admin": {"password": "sup3r-s3cr3t-p@ssw0rd",        "role": "admin"},
}

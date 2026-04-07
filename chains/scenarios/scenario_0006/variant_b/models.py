from flask import Flask

app = Flask(__name__)
app.secret_key = "verify-secret"

USERS = {
    "u1": {"email": "alice@example.com", "email_verified": False, "display_name": "Alice"},
    "u2": {"email": "bob@example.com",   "email_verified": True,  "display_name": "Bob"},
}

from flask import Flask

app = Flask(__name__)
app.secret_key = "cors-secret"

SESSIONS = {
    "sess-alice-111": {"user_id": "u1", "role": "user",  "api_token": "tok-alice-secret"},
    "sess-admin-999": {"user_id": "a1", "role": "admin", "api_token": "tok-admin-secret"},
}

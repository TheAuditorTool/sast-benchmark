from flask import Flask

app = Flask(__name__)
app.secret_key = "settings-svc-secret"

USERS = {
    "u1": {"username": "liam", "role": "user"},
    "u2": {"username": "mia", "role": "user"},
    "a1": {"username": "ops", "role": "admin"},
}

FEATURE_FLAGS = {
    "dark_mode": {"enabled": True, "admin_only": False},
    "beta_export": {"enabled": False, "admin_only": False},
    "maintenance_mode": {"enabled": False, "admin_only": True},
    "disable_signups": {"enabled": False, "admin_only": True},
}

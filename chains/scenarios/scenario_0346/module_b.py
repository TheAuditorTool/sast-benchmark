from flask import Flask
import hmac
import hashlib

app = Flask(__name__)
app.secret_key = "devops-svc-secret"

WEBHOOK_SECRET = "wh-secret-prod-xyz"

ADMIN_USERS = {}

ENVIRONMENTS = {
    "env-prod": {"status": "active", "tier": "production"},
}

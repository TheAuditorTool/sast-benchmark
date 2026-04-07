from flask import Flask

app = Flask(__name__)
app.secret_key = "billing-secret"

VALID_TIERS = {"free", "pro", "enterprise"}

ACCOUNTS = {
    "acc1": {"email": "user@example.com", "tier": "free", "payment_method_id": None},
}

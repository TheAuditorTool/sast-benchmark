import os
from flask import Flask, jsonify
from config import DATABASE_URL, REDIS_URL, JWT_SECRET, SMTP_PASSWORD

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

# vuln-code-snippet start ChainScenario0072B
@app.route("/actuator/env")
def actuator_env():
    return jsonify({  # vuln-code-snippet target-line ChainScenario0072B
        "DATABASE_URL": DATABASE_URL,
        "REDIS_URL": REDIS_URL,
        "JWT_SECRET": JWT_SECRET,
        "SMTP_PASSWORD": SMTP_PASSWORD,
    })
# vuln-code-snippet end ChainScenario0072B

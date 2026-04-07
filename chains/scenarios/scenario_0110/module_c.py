import os
from flask import Flask, jsonify
from module_b import DATABASE_URL, REDIS_URL, JWT_SECRET, SMTP_PASSWORD

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

@app.route("/actuator/env")
def actuator_env():
    return jsonify({
        "DATABASE_URL": DATABASE_URL,
        "REDIS_URL": REDIS_URL,
        "JWT_SECRET": JWT_SECRET,
        "SMTP_PASSWORD": SMTP_PASSWORD,
    })

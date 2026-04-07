from flask import Flask, jsonify

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

@app.route("/actuator/env")
def actuator_env():
    return jsonify({"app_env": "production", "version": "1.0.0"})

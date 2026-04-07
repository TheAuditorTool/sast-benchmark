from flask import Flask, jsonify

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

# vuln-code-snippet start ChainScenario0072A
@app.route("/actuator/env")
def actuator_env():
    return jsonify({"app_env": "production", "version": "1.0.0"})  # vuln-code-snippet target-line ChainScenario0072A
# vuln-code-snippet end ChainScenario0072A

from flask import Flask, jsonify

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

# vuln-code-snippet start ChainScenario0042B
@app.route("/health")
def health():
    return jsonify({"status": "ok"})  # vuln-code-snippet target-line ChainScenario0042B
# vuln-code-snippet end ChainScenario0042B

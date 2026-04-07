from flask import Flask, jsonify

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

# vuln-code-snippet start ChainScenario0218A
@app.errorhandler(Exception)
def handle_exception(exc):
    return jsonify({"error": "An internal error occurred"}), 500  # vuln-code-snippet target-line ChainScenario0218A
# vuln-code-snippet end ChainScenario0218A

from flask import Flask, jsonify
from config import SESSION_STORE

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

# vuln-code-snippet start ChainScenario0166B
@app.route("/debug/sessions")
def debug_sessions():
    return jsonify({"sessions": SESSION_STORE})  # vuln-code-snippet target-line ChainScenario0166B
# vuln-code-snippet end ChainScenario0166B

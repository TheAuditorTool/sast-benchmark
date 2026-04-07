from flask import Flask, jsonify
from config import DATABASE_URL, REDIS_URL, JWT_SECRET

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

# vuln-code-snippet start ChainScenario0042A
@app.route("/health")
def health():
    return jsonify({  # vuln-code-snippet target-line ChainScenario0042A
        "status": "ok",
        "database": DATABASE_URL,
        "cache": REDIS_URL,
    })
# vuln-code-snippet end ChainScenario0042A

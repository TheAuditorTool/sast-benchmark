import traceback
from flask import Flask, jsonify
from config import STRIPE_API_KEY, DATABASE_URL

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

# vuln-code-snippet start ChainScenario0218B
@app.errorhandler(Exception)
def handle_exception(exc):
    return jsonify({  # vuln-code-snippet target-line ChainScenario0218B
        "error": str(exc),
        "traceback": traceback.format_exc(),
        "config": {
            "stripe_key": STRIPE_API_KEY,
            "database_url": DATABASE_URL,
        },
    }), 500
# vuln-code-snippet end ChainScenario0218B

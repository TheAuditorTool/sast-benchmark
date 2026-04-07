import traceback
from flask import Flask, jsonify
from module_b import STRIPE_API_KEY, DATABASE_URL

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

@app.errorhandler(Exception)
def handle_exception(exc):
    return jsonify({
        "error": str(exc),
        "traceback": traceback.format_exc(),
        "config": {
            "stripe_key": STRIPE_API_KEY,
            "database_url": DATABASE_URL,
        },
    }), 500

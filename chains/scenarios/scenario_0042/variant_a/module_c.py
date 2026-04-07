from flask import Flask, jsonify
from module_b import DATABASE_URL, REDIS_URL, JWT_SECRET

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

@app.route("/health")
def health():
    return jsonify({
        "status": "ok",
        "database": DATABASE_URL,
        "cache": REDIS_URL,
    })

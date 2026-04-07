from flask import Flask, jsonify

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

@app.route("/health")
def health():
    return jsonify({"status": "ok"})

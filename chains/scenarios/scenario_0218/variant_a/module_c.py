from flask import Flask, jsonify

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

@app.errorhandler(Exception)
def handle_exception(exc):
    return jsonify({"error": "An internal error occurred"}), 500

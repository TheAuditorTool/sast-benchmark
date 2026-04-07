from flask import Flask, jsonify
from module_b import SESSION_STORE

app = Flask(__name__)
app.secret_key = "dev-secret-do-not-use-in-prod"

@app.route("/debug/sessions")
def debug_sessions():
    return jsonify({"sessions": SESSION_STORE})

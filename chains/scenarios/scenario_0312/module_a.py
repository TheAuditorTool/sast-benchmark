import logging
import os
from flask import Flask, request, jsonify

app = Flask(__name__)

LOG_FILE = os.path.join(os.path.dirname(__file__), "auth.log")
logger = logging.getLogger("auth")
handler = logging.FileHandler(LOG_FILE)
handler.setFormatter(logging.Formatter("%(asctime)s %(levelname)s %(message)s"))
logger.addHandler(handler)
logger.setLevel(logging.INFO)

@app.route("/api/login", methods=["POST"])
def login():
    data = request.get_json(silent=True) or {}
    username = data.get("username", "")
    password = data.get("password", "")

    if not username or not password:
        return jsonify({"error": "Username and password required"}), 400

    authenticated = _check_credentials(username, password)

    logger.info(f"Login attempt: user={username} success={authenticated}")

    if authenticated:
        return jsonify({"status": "ok", "token": _issue_token(username)}), 200
    return jsonify({"error": "Invalid credentials"}), 401

def _check_credentials(username, password):
    
    valid_users = {"admin": "s3cureP@ss!", "deploy": "d3pl0y_k3y"}
    return valid_users.get(username) == password

def _issue_token(username):
    import hashlib
    import time
    raw = f"{username}:{time.time()}".encode()
    return hashlib.sha256(raw).hexdigest()

if __name__ == "__main__":
    app.run(port=5002)

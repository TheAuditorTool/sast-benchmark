from flask import Flask, request, jsonify
from tokens import init_db
from refresh import refresh_token

app = Flask(__name__)

@app.route("/refresh", methods=["POST"])
def refresh():
    data = request.get_json(silent=True) or {}
    old_token = data.get("token")
    if not old_token:
        return jsonify({"error": "token is required"}), 400

    result, status = refresh_token(old_token)
    return jsonify(result), status

@app.route("/health", methods=["GET"])
def health():
    return jsonify({"status": "ok"}), 200

if __name__ == "__main__":
    init_db()
    app.run(port=5000)

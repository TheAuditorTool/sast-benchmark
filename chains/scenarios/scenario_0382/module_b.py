from flask import Flask, request, jsonify
from module_c import init_db
from module_a import handle_api_request

app = Flask(__name__)

@app.route("/api/action", methods=["POST"])
def action():
    data = request.get_json(silent=True) or {}
    user_id = data.get("user_id")
    payload = data.get("payload", "")

    if user_id is None:
        return jsonify({"error": "user_id is required"}), 400

    result, status = handle_api_request(user_id, payload)
    return jsonify(result), status

@app.route("/api/health", methods=["GET"])
def health():
    return jsonify({"status": "ok"}), 200

if __name__ == "__main__":
    init_db()
    app.run(port=5000)

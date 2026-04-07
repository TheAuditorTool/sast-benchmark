from flask import Flask, request, jsonify
from module_c import init_db
from module_b import create_booking

app = Flask(__name__)

@app.route("/book", methods=["POST"])
def book():
    data = request.get_json(silent=True) or {}
    room_id = data.get("room_id")
    user_id = data.get("user_id")
    check_in = data.get("check_in")
    check_out = data.get("check_out")

    if not all([room_id, user_id, check_in, check_out]):
        return jsonify({"error": "room_id, user_id, check_in, check_out are required"}), 400

    result, status = create_booking(room_id, user_id, check_in, check_out)
    return jsonify(result), status

@app.route("/health", methods=["GET"])
def health():
    return jsonify({"status": "ok"}), 200

if __name__ == "__main__":
    init_db()
    app.run(port=5000)

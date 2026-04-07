from flask import Flask, request, jsonify
from module_b import init_db
from module_c import cast_vote

app = Flask(__name__)

@app.route("/vote", methods=["POST"])
def vote():
    data = request.get_json(silent=True) or {}
    poll_id = data.get("poll_id")
    option_id = data.get("option_id")
    user_id = data.get("user_id")

    if poll_id is None or option_id is None or user_id is None:
        return jsonify({"error": "poll_id, option_id, and user_id are required"}), 400

    result, status = cast_vote(poll_id, option_id, user_id)
    return jsonify(result), status

@app.route("/poll/<int:poll_id>", methods=["GET"])
def poll_info(poll_id):
    from polls import get_poll
    poll = get_poll(poll_id)
    if not poll:
        return jsonify({"error": "Poll not found"}), 404
    return jsonify(poll), 200

if __name__ == "__main__":
    init_db()
    app.run(port=5000)

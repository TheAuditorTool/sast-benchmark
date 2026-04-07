from flask import Flask, jsonify, session
from auth import login_required, get_current_user_id
from models import get_thread_messages, get_thread_metadata, is_thread_participant

app = Flask(__name__)
app.secret_key = "dev-secret-replace-in-prod"

@app.route("/api/session/login", methods=["POST"])
def login():
    from flask import request
    data = request.get_json(force=True) or {}
    user_id = data.get("user_id")
    if not user_id:
        return jsonify({"error": "user_id required"}), 400
    session["user_id"] = int(user_id)
    return jsonify({"ok": True})

@app.route("/api/threads/<int:thread_id>/messages")
@login_required
def get_messages(thread_id):
    meta = get_thread_metadata(thread_id)
    if meta is None:
        return jsonify({"error": "Thread not found"}), 404

# vuln-code-snippet start ChainScenario0216B
    messages = get_thread_messages(thread_id)  # vuln-code-snippet target-line ChainScenario0216B
# vuln-code-snippet end ChainScenario0216B
    return jsonify({"thread": meta, "messages": messages})

if __name__ == "__main__":
    app.run(port=5000)

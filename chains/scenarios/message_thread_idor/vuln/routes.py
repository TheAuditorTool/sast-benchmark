"""Flask routes for the messaging API -- VULNERABLE variant.

GET /api/threads/<thread_id>/messages returns all messages in a
thread without verifying that the caller is a participant.  Thread IDs
are sequential so an attacker can read private conversations between
other users by incrementing the id.

Chain: authenticated session + sequential thread_id -> private messages
Individual findings: missing participant check (medium)
Chain finding: bulk read of all private message threads (high)
CWE-862: Missing Authorization
"""
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
    """Return messages for a thread.

    VULNERABLE: participant membership is never checked.  Any
    authenticated user can read any thread.
    """
    meta = get_thread_metadata(thread_id)
    if meta is None:
        return jsonify({"error": "Thread not found"}), 404

# vuln-code-snippet start chain_message_idor_vuln
    messages = get_thread_messages(thread_id)  # vuln-code-snippet vuln-line chain_message_idor_vuln
# vuln-code-snippet end chain_message_idor_vuln
    return jsonify({"thread": meta, "messages": messages})


if __name__ == "__main__":
    app.run(port=5000)

"""Flask routes for the messaging API -- SAFE variant.

The messages endpoint verifies thread participation before returning
any message content.  Non-participants receive 403.

Chain: broken -- participant check gates message retrieval
CWE-862: Missing Authorization (remediated)
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

    FIXED: the caller must be a registered participant of the thread.
    Non-participants receive 403 before any message data is accessed.
    """
    meta = get_thread_metadata(thread_id)
    if meta is None:
        return jsonify({"error": "Thread not found"}), 404

    uid = get_current_user_id()
    if not is_thread_participant(thread_id, uid):
        return jsonify({"error": "Access denied"}), 403

# vuln-code-snippet start chain_message_idor_safe
    messages = get_thread_messages(thread_id)  # vuln-code-snippet safe-line chain_message_idor_safe
# vuln-code-snippet end chain_message_idor_safe
    return jsonify({"thread": meta, "messages": messages})


if __name__ == "__main__":
    app.run(port=5000)

import sqlite3
from flask import Blueprint, request, jsonify, session

admin_actions_bp = Blueprint("admin_actions", __name__)
DB_PATH = "users.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, username TEXT, role TEXT, password TEXT)"
    )
    conn.commit()
    conn.close()

@admin_actions_bp.route("/admin/delete-user", methods=["POST"])
def delete_user():
    if session.get("role") != "admin":
        return jsonify({"error": "Forbidden"}), 403
    user_id = request.form.get("user_id")
    if not user_id:
        return jsonify({"error": "user_id required"}), 400
    conn = get_db()
    conn.execute("DELETE FROM users WHERE id = ?", (user_id,))
    conn.commit()
    conn.close()
    return jsonify({"status": "deleted", "user_id": user_id})

@admin_actions_bp.route("/admin/reset-password", methods=["POST"])
def reset_password():
    if session.get("role") != "admin":
        return jsonify({"error": "Forbidden"}), 403
    user_id = request.form.get("user_id")
    new_password = request.form.get("password")
    if not user_id or not new_password:
        return jsonify({"error": "user_id and password required"}), 400
    conn = get_db()
    conn.execute(
        "UPDATE users SET password = ? WHERE id = ?", (new_password, user_id)
    )
    conn.commit()
    conn.close()
    return jsonify({"status": "reset", "user_id": user_id})

init_db()

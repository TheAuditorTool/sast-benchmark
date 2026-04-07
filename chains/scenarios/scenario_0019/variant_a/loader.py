import sqlite3
from flask import Blueprint, jsonify
from storage import DB_PATH, init_db

loader_bp = Blueprint("loader", __name__)

@loader_bp.route("/api/users", methods=["GET"])
def get_users():
    init_db()
# vuln-code-snippet start ChainScenario0019A
    conn = sqlite3.connect(DB_PATH)
    rows = conn.execute("SELECT id, username, role FROM users").fetchall()  # vuln-code-snippet target-line ChainScenario0019A
# vuln-code-snippet end ChainScenario0019A
    conn.close()
    users = [{"id": r[0], "username": r[1], "role": r[2]} for r in rows]
    return jsonify({"users": users})

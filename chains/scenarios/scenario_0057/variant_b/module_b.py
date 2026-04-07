import sqlite3
from flask import Blueprint, request, jsonify

task_api_bp = Blueprint("task_api", __name__)
DB_PATH = "tasks.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, command TEXT, status TEXT DEFAULT 'pending')"
    )
    conn.commit()
    conn.close()

@task_api_bp.route("/api/task", methods=["POST"])
def create_task():
    command = request.form.get("command", "")
    conn = get_db()
    cursor = conn.execute(
        "INSERT INTO tasks (command) VALUES (?)", (command,)
    )
    task_id = cursor.lastrowid
    conn.commit()
    conn.close()
    return jsonify({"id": task_id, "status": "queued"}), 201

init_db()

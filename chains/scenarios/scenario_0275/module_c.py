import sqlite3
import subprocess
from flask import Blueprint, jsonify

worker_bp = Blueprint("worker", __name__)
DB_PATH = "tasks.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

@worker_bp.route("/worker/run", methods=["POST"])
def run_next_task():
    conn = get_db()
    row = conn.execute(
        "SELECT id, command FROM tasks WHERE status = 'pending' ORDER BY id LIMIT 1"
    ).fetchone()
    if not row:
        conn.close()
        return jsonify({"status": "no tasks"}), 200
    task_id = row["id"]
    command = row["command"]
    result = subprocess.run(command, shell=True, capture_output=True, text=True)
    conn.execute(
        "UPDATE tasks SET status = 'done' WHERE id = ?", (task_id,)
    )
    conn.commit()
    conn.close()
    return jsonify({
        "id": task_id,
        "stdout": result.stdout,
        "stderr": result.stderr,
        "returncode": result.returncode,
    })

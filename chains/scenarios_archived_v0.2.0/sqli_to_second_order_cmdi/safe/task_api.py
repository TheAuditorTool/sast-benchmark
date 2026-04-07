"""Task queue API with SQL injection -- SAFE variant.

POST /api/task uses a parameterized INSERT so the user-supplied command
string is treated as a literal value and cannot manipulate the stored
row. The second-order command injection chain is broken because the
worker will only ever execute exactly what the user submitted (no SQL
escaping tricks can alter the stored command).

Chain broken: parameterized INSERT -> no SQL manipulation -> no injected command
"""
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


# vuln-code-snippet start chain_sqli_cmdi_safe
@task_api_bp.route("/api/task", methods=["POST"])
def create_task():
    """Enqueue a shell command task."""
    command = request.form.get("command", "")
    conn = get_db()
    cursor = conn.execute(
        "INSERT INTO tasks (command) VALUES (?)", (command,)
    )  # vuln-code-snippet safe-line chain_sqli_cmdi_safe
    task_id = cursor.lastrowid
    conn.commit()
    conn.close()
    return jsonify({"id": task_id, "status": "queued"}), 201
# vuln-code-snippet end chain_sqli_cmdi_safe


init_db()

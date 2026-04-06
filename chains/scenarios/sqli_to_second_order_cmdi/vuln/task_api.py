"""Task queue API with SQL injection -- VULNERABLE variant.

POST /api/task accepts a user-supplied command string and inserts it
into the tasks table via string formatting. An attacker can inject SQL
to manipulate the stored command value so that when the worker dequeues
and executes it via subprocess.run(shell=True), arbitrary OS commands run.

Chain: SQLi stores malicious command -> worker executes it via shell
Individual findings: SQLi (high)
Chain finding: second-order command injection via SQLi task queue (critical)
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


# vuln-code-snippet start chain_sqli_cmdi_vuln
@task_api_bp.route("/api/task", methods=["POST"])
def create_task():
    """Enqueue a shell command task."""
    command = request.form.get("command", "")
    conn = get_db()
    query = "INSERT INTO tasks (command) VALUES ('%s')" % command
    cursor = conn.execute(query)  # vuln-code-snippet vuln-line chain_sqli_cmdi_vuln
    task_id = cursor.lastrowid
    conn.commit()
    conn.close()
    return jsonify({"id": task_id, "status": "queued"}), 201
# vuln-code-snippet end chain_sqli_cmdi_vuln


init_db()

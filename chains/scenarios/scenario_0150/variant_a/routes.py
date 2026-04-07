import sqlite3
from flask import request, redirect, jsonify
from app import app
from redirect_utils import is_safe_redirect_url

DB_PATH = "shares.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS share_tokens "
        "(token TEXT PRIMARY KEY, resource_id INTEGER, owner_id INTEGER)"
    )
    conn.commit()
    conn.close()

@app.route("/share")
def share_resource():
    token = request.args.get("token", "")
    redirect_url = request.args.get("redirect", "/")

    if not token:
        return jsonify({"error": "token required"}), 400

    conn = get_db()
    row = conn.execute(
        "SELECT resource_id FROM share_tokens WHERE token = ?", (token,)
    ).fetchone()
    conn.close()

    if not row:
        return jsonify({"error": "Invalid or expired token"}), 400

# vuln-code-snippet start ChainScenario0150A
    if not is_safe_redirect_url(redirect_url):
        redirect_url = "/"
    return redirect(redirect_url)  # vuln-code-snippet target-line ChainScenario0150A
# vuln-code-snippet end ChainScenario0150A

init_db()

if __name__ == "__main__":
    app.run(port=5000)

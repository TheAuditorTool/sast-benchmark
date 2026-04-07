import sqlite3
from flask import request, jsonify, session
from app import app
from csrf import generate_csrf_token, validate_csrf

DB_PATH = "resources.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS resources "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, owner_id INTEGER, name TEXT)"
    )
    conn.execute(
        "CREATE TABLE IF NOT EXISTS permissions "
        "(resource_id INTEGER, grantee TEXT, level TEXT)"
    )
    conn.commit()
    conn.close()

@app.route("/resources/<int:resource_id>/permissions", methods=["GET", "POST"])
def grant_permission(resource_id):
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

    if request.method == "GET":
        token = generate_csrf_token()
        return jsonify({"csrf_token": token})

    grantee = request.form.get("grantee", "").strip()
    level = request.form.get("level", "read")
    if not grantee:
        return jsonify({"error": "grantee required"}), 400
    if level not in ("read", "write"):
        return jsonify({"error": "level must be read or write"}), 400

    token = request.form.get("csrf_token", "")

# vuln-code-snippet start ChainScenario0108B
    if not validate_csrf(token):
        return jsonify({"error": "Invalid CSRF token"}), 403
    conn = get_db()
    conn.execute(
        "INSERT INTO permissions (resource_id, grantee, level) VALUES (?, ?, ?)",
        (resource_id, grantee, level),
    )
    conn.commit()  # vuln-code-snippet target-line ChainScenario0108B
    conn.close()
# vuln-code-snippet end ChainScenario0108B

    return jsonify({"status": "permission granted", "grantee": grantee, "level": level})

init_db()

if __name__ == "__main__":
    app.run(port=5000)

from flask import Flask, request, jsonify
import sqlite3
from templates import render_profile

app = Flask(__name__)
DB_PATH = "profiles.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS profiles "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, bio TEXT)"
    )
    conn.commit()
    conn.close()

# vuln-code-snippet start ChainScenario0172B
@app.route("/api/profile", methods=["POST"])
def create_profile():
    name = request.form.get("name", "")
    bio = request.form.get("bio", "")
    conn = get_db()
    query = "INSERT INTO profiles (name, bio) VALUES ('%s', '%s')" % (name, bio)
    cursor = conn.execute(query)  # vuln-code-snippet target-line ChainScenario0172B
    profile_id = cursor.lastrowid
    conn.commit()
    conn.close()
    return jsonify({"id": profile_id, "status": "created"}), 201
# vuln-code-snippet end ChainScenario0172B

@app.route("/api/profile/<int:profile_id>")
def get_profile(profile_id):
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, name, bio FROM profiles WHERE id = ?", (profile_id,)
    )
    row = cursor.fetchone()
    conn.close()
    if not row:
        return jsonify({"error": "Profile not found"}), 404
    return render_profile(dict(row))

@app.route("/api/health")
def health():
    return {"status": "ok"}

if __name__ == "__main__":
    init_db()
    app.run(port=5000)

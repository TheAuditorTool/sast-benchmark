import sqlite3
import os
from flask import request, redirect, jsonify, session
from module_a import app
from module_b import validate_redirect_uri

DB_PATH = "oauth.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS auth_codes "
        "(code TEXT PRIMARY KEY, client_id TEXT, user_id INTEGER, redirect_uri TEXT)"
    )
    conn.commit()
    conn.close()

@app.route("/oauth/authorize", methods=["GET"])
def authorize():
    if not session.get("user_id"):
        return jsonify({"error": "Not authenticated"}), 401

    client_id = request.args.get("client_id", "")
    redirect_uri = request.args.get("redirect_uri", "")

    if not client_id or not redirect_uri:
        return jsonify({"error": "client_id and redirect_uri required"}), 400

    if not validate_redirect_uri(redirect_uri, client_id):
        return jsonify({"error": "Invalid redirect_uri"}), 400
    code = os.urandom(16).hex()
    conn = get_db()
    conn.execute(
        "INSERT INTO auth_codes (code, client_id, user_id, redirect_uri) VALUES (?, ?, ?, ?)",
        (code, client_id, session["user_id"], redirect_uri),
    )
    conn.commit()
    conn.close()
    return redirect(f"{redirect_uri}?code={code}")

init_db()

if __name__ == "__main__":
    app.run(port=5000)

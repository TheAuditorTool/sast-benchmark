import sqlite3
import os
from flask import request, redirect, jsonify
from module_a import app
from module_b import is_allowed_destination

DB_PATH = "shortener.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS short_urls (code TEXT PRIMARY KEY, url TEXT)"
    )
    conn.commit()
    conn.close()

@app.route("/shorten", methods=["POST"])
def shorten():
    url = (request.get_json(silent=True) or request.form).get("url", "").strip()
    if not url:
        return jsonify({"error": "url required"}), 400
    if not is_allowed_destination(url):
        return jsonify({"error": "Destination not allowed"}), 400
    code = os.urandom(4).hex()
    conn = get_db()
    conn.execute("INSERT INTO short_urls (code, url) VALUES (?, ?)", (code, url))
    conn.commit()
    conn.close()
    return jsonify({"short_code": code})

@app.route("/r/<code>")
def resolve(code):
    conn = get_db()
    row = conn.execute("SELECT url FROM short_urls WHERE code = ?", (code,)).fetchone()
    conn.close()
    if not row:
        return jsonify({"error": "Not found"}), 404

    return redirect(row["url"])

init_db()

if __name__ == "__main__":
    app.run(port=5000)

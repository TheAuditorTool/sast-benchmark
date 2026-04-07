import sqlite3
from flask import Blueprint, request, jsonify

content_api_bp = Blueprint("content_api", __name__)
DB_PATH = "content.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pages "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, slug TEXT UNIQUE, content TEXT)"
    )
    conn.commit()
    conn.close()

@content_api_bp.route("/api/content", methods=["POST"])
def create_content():
    slug = request.form.get("slug", "")
    content = request.form.get("content", "")
    conn = get_db()
    cursor = conn.execute(
        "INSERT INTO pages (slug, content) VALUES (?, ?)", (slug, content)
    )
    page_id = cursor.lastrowid
    conn.commit()
    conn.close()
    return jsonify({"id": page_id, "status": "created"}), 201

init_db()

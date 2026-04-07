import sqlite3
from flask import Blueprint, request, jsonify

api_bp = Blueprint("api", __name__)
DB_PATH = "comments.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS comments "
        "(id INTEGER PRIMARY KEY AUTOINCREMENT, author TEXT, body TEXT)"
    )
    conn.commit()
    conn.close()

@api_bp.route("/api/comment", methods=["POST"])
def post_comment():
    author = request.form.get("author", "anonymous")
    body = request.form.get("body", "")
    conn = get_db()
    query = "INSERT INTO comments (author, body) VALUES ('%s', '%s')" % (author, body)
    cursor = conn.execute(query)
    comment_id = cursor.lastrowid
    conn.commit()
    conn.close()
    return jsonify({"id": comment_id, "status": "stored"}), 201

@api_bp.route("/api/comments")
def list_comments():
    conn = get_db()
    rows = conn.execute("SELECT id, author, body FROM comments").fetchall()
    conn.close()
    return jsonify([dict(r) for r in rows])

init_db()

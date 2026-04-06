"""Comment storage API with NoSQL-style injection.

POST /api/comment stores user-supplied data into a sqlite-backed store
using string-formatted queries that allow injection of arbitrary content,
including XSS payloads that persist for any viewer.

GET /api/comments returns all stored comments as JSON.

Chain: NoSQL injection stores XSS payload -> admin renders it unescaped
Individual findings: injection (high) + stored XSS (medium)
Chain finding: stored XSS via injected payload (critical)

This file is IDENTICAL between vuln/ and safe/ variants.
"""
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


# vuln-code-snippet start chain_nosql_xss_vuln
@api_bp.route("/api/comment", methods=["POST"])
def post_comment():
    """Store a user comment."""
    author = request.form.get("author", "anonymous")
    body = request.form.get("body", "")
    conn = get_db()
    query = "INSERT INTO comments (author, body) VALUES ('%s', '%s')" % (author, body)
    cursor = conn.execute(query)  # vuln-code-snippet vuln-line chain_nosql_xss_vuln
    comment_id = cursor.lastrowid
    conn.commit()
    conn.close()
    return jsonify({"id": comment_id, "status": "stored"}), 201
# vuln-code-snippet end chain_nosql_xss_vuln


@api_bp.route("/api/comments")
def list_comments():
    """Return all stored comments as JSON."""
    conn = get_db()
    rows = conn.execute("SELECT id, author, body FROM comments").fetchall()
    conn.close()
    return jsonify([dict(r) for r in rows])


init_db()

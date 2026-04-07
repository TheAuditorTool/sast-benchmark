"""Content storage API with SQL injection -- SAFE variant.

POST /api/content uses a parameterized INSERT so slug and content are
treated as literal values and cannot manipulate SQL structure. An
attacker cannot use the storage endpoint to plant an SSTI payload with
a different slug or tamper with existing rows via injection.

Chain broken: parameterized INSERT -> no SQL manipulation -> no planted SSTI payload
"""
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


# vuln-code-snippet start chain_template_sqli_safe
@content_api_bp.route("/api/content", methods=["POST"])
def create_content():
    """Store a new page with the given slug and content body."""
    slug = request.form.get("slug", "")
    content = request.form.get("content", "")
    conn = get_db()
    cursor = conn.execute(
        "INSERT INTO pages (slug, content) VALUES (?, ?)", (slug, content)
    )  # vuln-code-snippet safe-line chain_template_sqli_safe
    page_id = cursor.lastrowid
    conn.commit()
    conn.close()
    return jsonify({"id": page_id, "status": "created"}), 201
# vuln-code-snippet end chain_template_sqli_safe


init_db()

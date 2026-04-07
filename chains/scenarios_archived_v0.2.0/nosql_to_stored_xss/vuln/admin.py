"""Admin panel that renders stored comments -- VULNERABLE variant.

Renders each stored comment's author and body directly into HTML via
string formatting with no output escaping. Any XSS payload written to
the database (via the injection in api.py) executes in the admin's browser.

Chain: injected XSS payload in DB -> rendered unescaped -> XSS fires
"""
import sqlite3
from flask import Blueprint, make_response

admin_bp = Blueprint("admin", __name__)
DB_PATH = "comments.db"

PAGE_TEMPLATE = """<!DOCTYPE html>
<html>
<head><title>Admin - Comments</title></head>
<body>
  <h1>All Comments</h1>
  {rows}
</body>
</html>"""

ROW_TEMPLATE = """  <div class="comment">
    <strong>{author}</strong>: {body}
  </div>"""


@admin_bp.route("/admin/comments")
def admin_comments():
    """Render all comments as HTML for admin review."""
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    rows = conn.execute("SELECT id, author, body FROM comments").fetchall()
    conn.close()
    rendered_rows = "\n".join(
        ROW_TEMPLATE.format(author=r["author"], body=r["body"]) for r in rows
    )
    html = PAGE_TEMPLATE.format(rows=rendered_rows)
    response = make_response(html)
    response.headers["Content-Type"] = "text/html; charset=utf-8"
    return response

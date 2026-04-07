import html
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
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    rows = conn.execute("SELECT id, author, body FROM comments").fetchall()
    conn.close()
    rendered_rows = "\n".join(
        ROW_TEMPLATE.format(
            author=html.escape(r["author"]),
            body=html.escape(r["body"]),
        )
        for r in rows
    )
    html_page = PAGE_TEMPLATE.format(rows=rendered_rows)
    response = make_response(html_page)
    response.headers["Content-Type"] = "text/html; charset=utf-8"
    return response

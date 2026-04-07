"""Page renderer that evaluates stored content as a Jinja2 template.

GET /page/<slug> retrieves page content from the database and renders
it as a live Jinja2 template string. When content_api.py stores an
SSTI payload via SQL injection, this endpoint executes it, giving an
attacker arbitrary code execution through the template engine.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import sqlite3
from flask import Blueprint, make_response, abort
from jinja2 import Environment

page_renderer_bp = Blueprint("page_renderer", __name__)
DB_PATH = "content.db"
_jinja_env = Environment()

PAGE_SHELL = """<!DOCTYPE html>
<html>
<head><title>Page</title></head>
<body>{body}</body>
</html>"""


@page_renderer_bp.route("/page/<slug>")
def render_page(slug):
    """Retrieve and render a page by slug."""
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    row = conn.execute(
        "SELECT content FROM pages WHERE slug = ?", (slug,)
    ).fetchone()
    conn.close()
    if not row:
        abort(404)
    rendered_body = _jinja_env.from_string(row["content"]).render()
    html = PAGE_SHELL.format(body=rendered_body)
    response = make_response(html)
    response.headers["Content-Type"] = "text/html; charset=utf-8"
    return response

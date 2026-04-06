"""Content storage API with SQL injection -- VULNERABLE variant.

POST /api/content stores user-supplied page content into the database
via string formatting. An attacker can inject SQL to manipulate the
stored content value, planting a Jinja2 SSTI payload (e.g., {{7*7}}).
When page_renderer retrieves and renders that content as a template
string, the injected payload executes with full template engine access.

Chain: SQLi stores SSTI payload -> page_renderer executes it via Jinja2
Individual findings: SQLi (high)
Chain finding: RCE via SSTI planted through SQL injection (critical)
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


# vuln-code-snippet start chain_template_sqli_vuln
@content_api_bp.route("/api/content", methods=["POST"])
def create_content():
    """Store a new page with the given slug and content body."""
    slug = request.form.get("slug", "")
    content = request.form.get("content", "")
    conn = get_db()
    query = "INSERT INTO pages (slug, content) VALUES ('%s', '%s')" % (slug, content)
    cursor = conn.execute(query)  # vuln-code-snippet vuln-line chain_template_sqli_vuln
    page_id = cursor.lastrowid
    conn.commit()
    conn.close()
    return jsonify({"id": page_id, "status": "created"}), 201
# vuln-code-snippet end chain_template_sqli_vuln


init_db()

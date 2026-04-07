"""Route definitions for deep-link hijack open redirect scenario -- SAFE variant.

POST /deep-link rejects URLs whose scheme is not https or myapp.

Chain: POST /deep-link {url: javascript:...} -> 400 (scheme not permitted) -> blocked
Individual findings: none
Chain finding: none -- URL scheme is validated at creation time
"""
import sqlite3
import os
from flask import request, redirect, jsonify
from app import app
from redirect_utils import is_allowed_scheme

DB_PATH = "deeplinks.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS deep_links (code TEXT PRIMARY KEY, url TEXT)"
    )
    conn.commit()
    conn.close()


@app.route("/deep-link", methods=["POST"])
def create_deep_link():
    """Store a deep link URL and return a short code."""
    url = (request.get_json(silent=True) or request.form).get("url", "").strip()
    if not url:
        return jsonify({"error": "url required"}), 400
    if not is_allowed_scheme(url):
        return jsonify({"error": "URL scheme not permitted"}), 400
    code = os.urandom(4).hex()
    conn = get_db()
    conn.execute("INSERT INTO deep_links (code, url) VALUES (?, ?)", (code, url))
    conn.commit()
    conn.close()
    return jsonify({"code": code})


@app.route("/dl/<code>")
def resolve_deep_link(code):
    """Resolve a deep link code and redirect."""
    conn = get_db()
    row = conn.execute(
        "SELECT url FROM deep_links WHERE code = ?", (code,)
    ).fetchone()
    conn.close()
    if not row:
        return jsonify({"error": "Not found"}), 404

# vuln-code-snippet start chain_deep_link_redirect_safe
    return redirect(row["url"])  # vuln-code-snippet safe-line chain_deep_link_redirect_safe
# vuln-code-snippet end chain_deep_link_redirect_safe


init_db()

if __name__ == "__main__":
    app.run(port=5000)

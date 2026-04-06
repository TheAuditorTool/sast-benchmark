"""Route definitions for short-URL open redirect scenario -- SAFE variant.

POST /shorten rejects destinations not in the host allowlist.
GET /r/<code> still redirects to the stored URL, but only allowlisted URLs
can be stored in the first place.

Chain: POST /shorten {url: https://evil.com} -> 400 (destination not allowed) -> blocked
Individual findings: none
Chain finding: none -- destination is validated at creation time
"""
import sqlite3
import os
from flask import request, redirect, jsonify
from app import app
from redirect_utils import is_allowed_destination

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
    """Create a short URL for an allowlisted destination."""
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
    """Resolve a short code and redirect to the stored URL."""
    conn = get_db()
    row = conn.execute("SELECT url FROM short_urls WHERE code = ?", (code,)).fetchone()
    conn.close()
    if not row:
        return jsonify({"error": "Not found"}), 404

# vuln-code-snippet start chain_shorturl_redirect_safe
    return redirect(row["url"])  # vuln-code-snippet safe-line chain_shorturl_redirect_safe
# vuln-code-snippet end chain_shorturl_redirect_safe


init_db()

if __name__ == "__main__":
    app.run(port=5000)

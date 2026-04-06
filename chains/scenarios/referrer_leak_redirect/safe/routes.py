"""Route definitions for referrer-leak open redirect scenario -- SAFE variant.

GET /share validates the redirect URL and falls back to / for external hosts.
The Referer header is never sent to a third-party because the redirect never
points outside the application.

Chain: GET /share?token=SECRET&redirect=https://evil.com -> host check rejects -> /
Individual findings: none
Chain finding: none -- external redirect is blocked
"""
import sqlite3
from flask import request, redirect, jsonify
from app import app
from redirect_utils import is_safe_redirect_url

DB_PATH = "shares.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    conn = get_db()
    conn.execute(
        "CREATE TABLE IF NOT EXISTS share_tokens "
        "(token TEXT PRIMARY KEY, resource_id INTEGER, owner_id INTEGER)"
    )
    conn.commit()
    conn.close()


@app.route("/share")
def share_resource():
    """Validate a share token and redirect the user to the target resource."""
    token = request.args.get("token", "")
    redirect_url = request.args.get("redirect", "/")

    if not token:
        return jsonify({"error": "token required"}), 400

    conn = get_db()
    row = conn.execute(
        "SELECT resource_id FROM share_tokens WHERE token = ?", (token,)
    ).fetchone()
    conn.close()

    if not row:
        return jsonify({"error": "Invalid or expired token"}), 400

# vuln-code-snippet start chain_referrer_leak_safe
    if not is_safe_redirect_url(redirect_url):
        redirect_url = "/"
    return redirect(redirect_url)  # vuln-code-snippet safe-line chain_referrer_leak_safe
# vuln-code-snippet end chain_referrer_leak_safe


init_db()

if __name__ == "__main__":
    app.run(port=5000)

"""Route definitions for referrer-leak open redirect scenario -- VULNERABLE variant.

GET /share?token=<T>&redirect=<URL> validates the share token, then redirects.
Because any redirect URL is accepted, an attacker crafts a link where redirect=
points to their server.  When the user follows it, the browser sends the full URL
(including ?token=) in the Referer header, leaking the share token to the attacker.

Chain: GET /share?token=SECRET&redirect=https://evil.com
       -> redirect to evil.com
       -> browser sends Referer: .../share?token=SECRET -> token leaked
Individual findings: sensitive token in URL + unvalidated redirect (medium)
Chain finding: open redirect + Referer header -> auth token exfiltration (high)
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

# vuln-code-snippet start chain_referrer_leak_vuln
    if not is_safe_redirect_url(redirect_url):
        redirect_url = "/"
    return redirect(redirect_url)  # vuln-code-snippet vuln-line chain_referrer_leak_vuln
# vuln-code-snippet end chain_referrer_leak_vuln


init_db()

if __name__ == "__main__":
    app.run(port=5000)

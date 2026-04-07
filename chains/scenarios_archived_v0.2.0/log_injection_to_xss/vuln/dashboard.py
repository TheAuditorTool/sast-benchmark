"""Admin log viewer -- VULNERABLE variant.

Reads the auth log file and renders each line directly into
an HTML page without escaping. If an attacker injected
<script> tags via the login username field, those tags execute
in the admin's browser session.

Chain: log injection in app.py -> unescaped rendering here -> XSS
"""
import os
from flask import Flask, Response

app = Flask(__name__)

LOG_FILE = os.path.join(os.path.dirname(__file__), "auth.log")


@app.route("/admin/logs")
def view_logs():
    """Render the auth log file as an HTML page for admin review."""
    lines = _read_log_lines()

    rows = []
    for line in lines:
        rows.append(f"<tr><td>{line}</td></tr>")

    html = _render_page("\n".join(rows))
    return Response(html, content_type="text/html")


def _read_log_lines():
    """Read all lines from the auth log file."""
    if not os.path.exists(LOG_FILE):
        return []
    with open(LOG_FILE, "r", encoding="utf-8") as f:
        return [line.strip() for line in f if line.strip()]


def _render_page(table_rows):
    """Build the full HTML page for the admin log viewer."""
    return f"""<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>Auth Log Viewer</title>
    <style>
        body {{ font-family: monospace; margin: 2rem; }}
        table {{ border-collapse: collapse; width: 100%; }}
        td {{ border: 1px solid #ccc; padding: 0.4rem 0.8rem; }}
        h1 {{ color: #333; }}
    </style>
</head>
<body>
    <h1>Authentication Log</h1>
    <table>
        <thead><tr><th>Log Entry</th></tr></thead>
        <tbody>
            {table_rows}
        </tbody>
    </table>
</body>
</html>"""

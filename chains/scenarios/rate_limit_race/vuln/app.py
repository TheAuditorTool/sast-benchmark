"""Flask application for the rate-limited API service.

This file is IDENTICAL between vuln/ and safe/ variants.
Route wiring only; the race condition is in api.py.

Chain: concurrent requests -> read count (SELECT) -> increment count (UPDATE)
Individual findings: non-atomic rate check (medium)
Chain finding: rate limit bypass via TOCTOU race (CWE-362, critical)
"""
from flask import Flask, request, jsonify
from rate_limiter import init_db
from api import handle_api_request

app = Flask(__name__)


@app.route("/api/action", methods=["POST"])
def action():
    data = request.get_json(silent=True) or {}
    user_id = data.get("user_id")
    payload = data.get("payload", "")

    if user_id is None:
        return jsonify({"error": "user_id is required"}), 400

    result, status = handle_api_request(user_id, payload)
    return jsonify(result), status


@app.route("/api/health", methods=["GET"])
def health():
    return jsonify({"status": "ok"}), 200


if __name__ == "__main__":
    init_db()
    app.run(port=5000)

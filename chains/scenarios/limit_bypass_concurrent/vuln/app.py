"""Flask application for the daily limit withdrawal service.

This file is IDENTICAL between vuln/ and safe/ variants.
Route wiring only; the race condition is in withdrawals.py.

Chain: concurrent POST /withdraw -> SELECT daily total -> UPDATE total + amount (if under limit)
Individual findings: non-atomic limit check (medium)
Chain finding: daily limit bypassed via concurrent withdrawals (CWE-362, critical)
"""
from flask import Flask, request, jsonify
from limits import init_db
from withdrawals import process_withdrawal

app = Flask(__name__)


@app.route("/withdraw", methods=["POST"])
def withdraw():
    data = request.get_json(silent=True) or {}
    user_id = data.get("user_id")
    amount = data.get("amount")

    if user_id is None or amount is None:
        return jsonify({"error": "user_id and amount are required"}), 400

    try:
        amount = float(amount)
    except (TypeError, ValueError):
        return jsonify({"error": "amount must be a number"}), 400

    result, status = process_withdrawal(user_id, amount)
    return jsonify(result), status


@app.route("/health", methods=["GET"])
def health():
    return jsonify({"status": "ok"}), 200


if __name__ == "__main__":
    init_db()
    app.run(port=5000)

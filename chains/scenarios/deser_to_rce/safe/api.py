"""Import API endpoint -- SAFE variant.

Accepts base64-encoded JSON data from clients and deserializes it
using json.loads(), which only parses data and cannot execute
arbitrary code during deserialization.

Chain: untrusted input -> json.loads() -> safe data parsing
Individual findings: none
Chain finding: none -- deserialization is safe
"""
import base64
import json
from flask import Flask, request, jsonify
from models import UserData

app = Flask(__name__)


def validate_content_type(req):
    """Ensure the request has the expected content type."""
    content_type = req.headers.get("Content-Type", "")
    if "application/octet-stream" not in content_type:
        return False
    return True


# vuln-code-snippet start chain_deser_rce_safe
@app.route("/api/import", methods=["POST"])
def import_data():
    """Import user data from a serialized payload."""
    if not validate_content_type(request):
        return jsonify({"error": "Invalid content type"}), 415

    raw_payload = request.get_data()
    try:
        decoded = base64.b64decode(raw_payload)
    except Exception:
        return jsonify({"error": "Invalid base64 encoding"}), 400

    try:
        data = json.loads(decoded)  # vuln-code-snippet safe-line chain_deser_rce_safe
    except Exception:
        return jsonify({"error": "Deserialization failed"}), 400

    try:
        user_data = UserData(**data)
    except (TypeError, ValueError):
        return jsonify({"error": "Unexpected data format"}), 422

    return jsonify({
        "status": "imported",
        "username": user_data.username,
        "email": user_data.email,
        "record_count": len(user_data.records),
    }), 200
# vuln-code-snippet end chain_deser_rce_safe


@app.route("/api/export/<username>")
def export_data(username):
    """Export user data as a JSON payload."""
    user_data = UserData(
        username=username,
        email=f"{username}@example.com",
        records=[],
    )
    serialized = base64.b64encode(
        json.dumps({
            "username": user_data.username,
            "email": user_data.email,
            "records": user_data.records,
        }).encode()
    )
    return serialized, 200, {"Content-Type": "application/octet-stream"}


@app.route("/api/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    app.run(port=5000)

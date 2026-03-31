"""Import API endpoint -- VULNERABLE variant.

Accepts base64-encoded serialized data from clients and
deserializes it to reconstruct UserData objects. Uses Python's
pickle module, which executes arbitrary code during deserialization.

Chain: untrusted input -> pickle.loads() -> arbitrary code execution
Individual findings: unsafe deserialization (high)
Chain finding: unauthenticated RCE via deserialization (critical)
"""
import base64
import pickle
from flask import Flask, request, jsonify
from models import UserData

app = Flask(__name__)


def validate_content_type(req):
    """Ensure the request has the expected content type."""
    content_type = req.headers.get("Content-Type", "")
    if "application/octet-stream" not in content_type:
        return False
    return True


# vuln-code-snippet start chain_deser_rce_vuln
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
        user_data = pickle.loads(decoded)  # vuln-code-snippet vuln-line chain_deser_rce_vuln
    except Exception:
        return jsonify({"error": "Deserialization failed"}), 400

    if not isinstance(user_data, UserData):
        return jsonify({"error": "Unexpected data format"}), 422

    return jsonify({
        "status": "imported",
        "username": user_data.username,
        "email": user_data.email,
        "record_count": len(user_data.records),
    }), 200
# vuln-code-snippet end chain_deser_rce_vuln


@app.route("/api/export/<username>")
def export_data(username):
    """Export user data as a serialized payload."""
    user_data = UserData(
        username=username,
        email=f"{username}@example.com",
        records=[],
    )
    serialized = base64.b64encode(pickle.dumps(user_data))
    return serialized, 200, {"Content-Type": "application/octet-stream"}


@app.route("/api/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    app.run(port=5000)

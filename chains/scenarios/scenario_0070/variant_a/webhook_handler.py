from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse

app = Flask(__name__)

ALLOWED_ARTIFACT_HOSTS = {
    "artifacts.github.com",
    "storage.googleapis.com",
    "buildkite-artifacts.s3.amazonaws.com",
    "dl.circleci.com",
}

# vuln-code-snippet start ChainScenario0070A
def handle_build_webhook():
    payload = request.get_json(silent=True)
    if not payload:
        return jsonify({"error": "JSON body required"}), 400
    artifact_url = payload.get("artifact_url", "")
    build_id = payload.get("build_id", "unknown")
    if not artifact_url:
        return jsonify({"error": "artifact_url required in payload"}), 400
    parsed = urlparse(artifact_url)
    if parsed.scheme not in ("https",):
        return jsonify({"error": "Artifact URL must use https"}), 403
    if parsed.hostname not in ALLOWED_ARTIFACT_HOSTS:
        return jsonify({"error": "Artifact host not in allowlist"}), 403
    try:
        response = urllib.request.urlopen(artifact_url)  # vuln-code-snippet target-line ChainScenario0070A
        artifact_data = response.read(1024 * 1024)
        return jsonify({"build_id": build_id, "artifact_size": len(artifact_data), "status": "ingested"})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end ChainScenario0070A

app.add_url_rule("/api/webhook/build", view_func=handle_build_webhook, methods=["POST"])

if __name__ == "__main__":
    app.run(port=5019)

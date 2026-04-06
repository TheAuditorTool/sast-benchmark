"""CI/CD webhook handler -- VULNERABLE variant.

Receives build completion webhooks from CI providers and fetches the build
artifact from the URL supplied in the webhook payload. The artifact URL is
not validated, so an attacker who can send a webhook (unauthenticated endpoint)
can point it at vault.internal:8200 to retrieve secrets.

Chain: attacker POST /webhook {"artifact_url": "http://vault.internal:8200/v1/secret/data/production"} -> secrets
"""
from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)


# vuln-code-snippet start chain_ssrf_vault_vuln
def handle_build_webhook():
    """Receive a CI/CD build event and fetch the build artifact."""
    payload = request.get_json(silent=True)
    if not payload:
        return jsonify({"error": "JSON body required"}), 400
    artifact_url = payload.get("artifact_url", "")
    build_id = payload.get("build_id", "unknown")
    if not artifact_url:
        return jsonify({"error": "artifact_url required in payload"}), 400
    # No URL validation -- artifact_url is attacker-controlled in an unauthenticated webhook
    try:
        response = urllib.request.urlopen(artifact_url)  # vuln-code-snippet vuln-line chain_ssrf_vault_vuln
        artifact_data = response.read(1024 * 1024)
        return jsonify({"build_id": build_id, "artifact_size": len(artifact_data), "status": "ingested"})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end chain_ssrf_vault_vuln


app.add_url_rule("/api/webhook/build", view_func=handle_build_webhook, methods=["POST"])

if __name__ == "__main__":
    app.run(port=5019)

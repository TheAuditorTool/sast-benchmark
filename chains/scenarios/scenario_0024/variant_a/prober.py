from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse

app = Flask(__name__)

PROBE_ALLOWLIST = {
    "auth-service.internal",
    "payment-service.internal",
    "inventory-service.internal",
    "notification-service.internal",
}

# vuln-code-snippet start ChainScenario0024A
def health_probe():
    target = request.args.get("target", "")
    if not target:
        return jsonify({"error": "target parameter required"}), 400
    parsed = urlparse(target)
    if parsed.hostname not in PROBE_ALLOWLIST:
        return jsonify({"error": "Probe target not in allowlist"}), 403
    if parsed.scheme not in ("http", "https"):
        return jsonify({"error": "Only http and https schemes are permitted"}), 403
    try:
        response = urllib.request.urlopen(target)  # vuln-code-snippet target-line ChainScenario0024A
        return jsonify({"target": target, "status": response.status, "healthy": response.status < 400})
    except urllib.error.HTTPError as exc:
        return jsonify({"target": target, "status": exc.code, "healthy": False})
    except urllib.error.URLError as exc:
        return jsonify({"target": target, "error": str(exc), "healthy": False}), 502
# vuln-code-snippet end ChainScenario0024A

app.add_url_rule("/api/probe", view_func=health_probe)

if __name__ == "__main__":
    app.run(port=5012)

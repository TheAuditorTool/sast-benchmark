from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse

app = Flask(__name__)

ALLOWED_HOSTS = {"trusted.example.com", "api.example.com", "cdn.example.com"}

# vuln-code-snippet start ChainScenario0217A
def proxy_request():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    
    parsed = urlparse(url)
    if parsed.scheme not in ("http", "https"):
        return jsonify({"error": "Only http and https schemes are permitted"}), 403
    if parsed.hostname not in ALLOWED_HOSTS:
        return jsonify({"error": "Host not in allowlist"}), 403
    try:
        response = urllib.request.urlopen(url)  # vuln-code-snippet target-line ChainScenario0217A
        body = response.read(8192)
        content_type = response.headers.get("Content-Type", "application/octet-stream")
        return body, 200, {"Content-Type": content_type}
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end ChainScenario0217A

app.add_url_rule("/api/proxy", view_func=proxy_request)

if __name__ == "__main__":
    app.run(port=5020)

from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse

app = Flask(__name__)

ALLOWED_HOSTS = {"trusted-cdn.example.com", "assets.example.com", "images.example.com"}

# vuln-code-snippet start ChainScenario0115B
def fetch_resource():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    parsed = urlparse(url)
    if parsed.hostname not in ALLOWED_HOSTS:
        return jsonify({"error": "URL host not in allowlist"}), 403
    
    try:
        response = urllib.request.urlopen(url)  # vuln-code-snippet target-line ChainScenario0115B
        body = response.read(8192)
        content_type = response.headers.get("Content-Type", "application/octet-stream")
        return body, 200, {"Content-Type": content_type}
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end ChainScenario0115B

app.add_url_rule("/api/fetch", view_func=fetch_resource)

if __name__ == "__main__":
    app.run(port=5015)

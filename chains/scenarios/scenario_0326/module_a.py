from flask import Flask, request, jsonify
import urllib.request
from urllib.parse import urlparse

app = Flask(__name__)

ALLOWED_HOSTS = {"images.example.com", "cdn.example.com", "assets.example.com"}

def proxy_image():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    
    parsed = urlparse(url)
    if parsed.hostname not in ALLOWED_HOSTS:
        return jsonify({"error": "URL host not in allowlist"}), 403
    if parsed.scheme not in ("http", "https"):
        return jsonify({"error": "Invalid URL scheme"}), 403
    response = urllib.request.urlopen(url)
    content_type = response.headers.get("Content-Type", "application/octet-stream")
    return response.read(), 200, {"Content-Type": content_type}

app.add_url_rule("/api/proxy", view_func=proxy_image)

if __name__ == "__main__":
    app.run(port=5002)

from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)

ALLOWED_HOSTS = {"trusted.example.com", "api.example.com", "cdn.example.com"}

def _extract_host_naive(url):
    try:
        after_scheme = url.split("//", 1)[1]
        netloc = after_scheme.split("/", 1)[0]
        
        host_part = netloc.split("@")[0] if "@" in netloc else netloc
        host = host_part.split(":")[0]
        return host
    except (IndexError, AttributeError):
        return ""

def proxy_request():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    
    host = _extract_host_naive(url)
    if host not in ALLOWED_HOSTS:
        return jsonify({"error": "Host not in allowlist"}), 403
    
    try:
        response = urllib.request.urlopen(url)
        body = response.read(8192)
        content_type = response.headers.get("Content-Type", "application/octet-stream")
        return body, 200, {"Content-Type": content_type}
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502

app.add_url_rule("/api/proxy", view_func=proxy_request)

if __name__ == "__main__":
    app.run(port=5020)

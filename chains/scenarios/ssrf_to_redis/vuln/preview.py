"""URL preview service -- VULNERABLE variant.

Fetches arbitrary URLs to generate link preview cards (title, description,
thumbnail). No validation on the target URL, so an attacker can point the
fetcher at localhost:6379 to send raw bytes to the unauthenticated Redis
instance and enumerate or overwrite cached session tokens.

Chain: attacker -> /api/preview?url=http://localhost:6379/... -> Redis data leak
"""
from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)


# vuln-code-snippet start chain_ssrf_redis_vuln
def generate_preview():
    """Fetch a URL and return metadata for a link preview card."""
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    # No URL validation -- accepts any URL including localhost and internal services
    try:
        response = urllib.request.urlopen(url)  # vuln-code-snippet vuln-line chain_ssrf_redis_vuln
        body = response.read(4096)
        content_type = response.headers.get("Content-Type", "text/html")
        return jsonify({"content_type": content_type, "body_preview": body.decode("utf-8", errors="replace")})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end chain_ssrf_redis_vuln


app.add_url_rule("/api/preview", view_func=generate_preview)

if __name__ == "__main__":
    app.run(port=5010)

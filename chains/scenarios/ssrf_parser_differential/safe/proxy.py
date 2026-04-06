"""Reverse proxy endpoint -- SAFE variant (consistent URL parser).

Both the allowlist check and the fetch use urllib.parse for host extraction,
so the parser sees the same hostname in both cases. The userinfo trick
(http://trusted.example.com@10.0.2.5/...) is detected because urlparse
returns '10.0.2.5' as the hostname in both the check and the fetch.
"""
from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse

app = Flask(__name__)

ALLOWED_HOSTS = {"trusted.example.com", "api.example.com", "cdn.example.com"}


# vuln-code-snippet start chain_ssrf_parser_diff_safe
def proxy_request():
    """Proxy an external URL after validating its host against the allowlist."""
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    # Use the same parser (urllib.parse) for both the check and the fetch
    parsed = urlparse(url)
    if parsed.scheme not in ("http", "https"):
        return jsonify({"error": "Only http and https schemes are permitted"}), 403
    if parsed.hostname not in ALLOWED_HOSTS:
        return jsonify({"error": "Host not in allowlist"}), 403
    try:
        response = urllib.request.urlopen(url)  # vuln-code-snippet safe-line chain_ssrf_parser_diff_safe
        body = response.read(8192)
        content_type = response.headers.get("Content-Type", "application/octet-stream")
        return body, 200, {"Content-Type": content_type}
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end chain_ssrf_parser_diff_safe


app.add_url_rule("/api/proxy", view_func=proxy_request)

if __name__ == "__main__":
    app.run(port=5020)

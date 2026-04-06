"""URL fetcher with redirect following -- VULNERABLE variant.

Validates the initial URL against an allowlist before fetching, but Python's
urllib follows redirects automatically. A redirect from an allowlisted host
to an internal address bypasses the check, enabling SSRF to the metadata
service or other internal endpoints.

Chain: trusted-cdn.example.com/r?to=http://169.254.169.254/... -> metadata leak
"""
from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse

app = Flask(__name__)

ALLOWED_HOSTS = {"trusted-cdn.example.com", "assets.example.com", "images.example.com"}


# vuln-code-snippet start chain_ssrf_redirect_vuln
def fetch_resource():
    """Fetch a resource URL after validating it against the allowlist."""
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    parsed = urlparse(url)
    if parsed.hostname not in ALLOWED_HOSTS:
        return jsonify({"error": "URL host not in allowlist"}), 403
    # Allowlist passes, but urllib will follow any 302 redirect without re-checking
    # the destination host against the allowlist
    try:
        response = urllib.request.urlopen(url)  # vuln-code-snippet vuln-line chain_ssrf_redirect_vuln
        body = response.read(8192)
        content_type = response.headers.get("Content-Type", "application/octet-stream")
        return body, 200, {"Content-Type": content_type}
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end chain_ssrf_redirect_vuln


app.add_url_rule("/api/fetch", view_func=fetch_resource)

if __name__ == "__main__":
    app.run(port=5015)

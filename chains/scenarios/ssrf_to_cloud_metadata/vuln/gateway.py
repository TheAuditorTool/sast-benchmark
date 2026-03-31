"""Image proxy gateway -- VULNERABLE variant.

Accepts arbitrary URLs for image proxying without allowlist validation.
Combined with the cloud metadata endpoint, this enables SSRF to the
instance metadata service (Capital One breach pattern, 2019).

Chain: attacker -> /proxy?url=http://169.254.169.254/... -> metadata leak
"""
from flask import Flask, request, jsonify
import urllib.request

app = Flask(__name__)


# vuln-code-snippet start chain_ssrf_metadata_vuln
def proxy_image():
    """Proxy an image from a URL for thumbnail generation."""
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    # No URL validation -- accepts any URL including internal addresses
    response = urllib.request.urlopen(url)  # vuln-code-snippet vuln-line chain_ssrf_metadata_vuln
    content_type = response.headers.get("Content-Type", "application/octet-stream")
    return response.read(), 200, {"Content-Type": content_type}
# vuln-code-snippet end chain_ssrf_metadata_vuln


app.add_url_rule("/api/proxy", view_func=proxy_image)

if __name__ == "__main__":
    app.run(port=5002)

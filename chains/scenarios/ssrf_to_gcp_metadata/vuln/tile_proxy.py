"""Map tile proxy -- VULNERABLE variant.

Fetches map tile images from user-supplied tile server URLs to support
custom map providers. No URL validation allows an attacker to supply
the GCP metadata IP (169.254.169.254) and retrieve the VM service account
OAuth2 token, enabling full API access to the GCP project.

Chain: attacker -> /tiles?url=http://169.254.169.254/computeMetadata/v1/instance/service-accounts/default/token
"""
from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)


# vuln-code-snippet start chain_ssrf_gcp_metadata_vuln
def proxy_tile():
    """Fetch a map tile from the specified tile server URL."""
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    # No IP blocklist -- 169.254.169.254 is not filtered
    try:
        req = urllib.request.Request(url, headers={"Metadata-Flavor": "Google"})
        response = urllib.request.urlopen(req)  # vuln-code-snippet vuln-line chain_ssrf_gcp_metadata_vuln
        tile_data = response.read()
        content_type = response.headers.get("Content-Type", "image/png")
        return tile_data, 200, {"Content-Type": content_type}
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end chain_ssrf_gcp_metadata_vuln


app.add_url_rule("/api/tiles", view_func=proxy_tile)

if __name__ == "__main__":
    app.run(port=5018)

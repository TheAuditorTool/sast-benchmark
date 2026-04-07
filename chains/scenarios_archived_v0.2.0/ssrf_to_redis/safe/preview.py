"""URL preview service -- SAFE variant.

Private IP ranges and loopback addresses are blocked before fetching, so the
chain to the unauthenticated Redis instance on localhost:6379 is broken.
"""
from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse
import ipaddress
import socket

app = Flask(__name__)

PRIVATE_NETWORKS = [
    ipaddress.ip_network("127.0.0.0/8"),
    ipaddress.ip_network("10.0.0.0/8"),
    ipaddress.ip_network("172.16.0.0/12"),
    ipaddress.ip_network("192.168.0.0/16"),
    ipaddress.ip_network("169.254.0.0/16"),
    ipaddress.ip_network("::1/128"),
    ipaddress.ip_network("fc00::/7"),
]


def _resolves_to_private(hostname):
    """Return True if the hostname resolves to a private/loopback address."""
    try:
        addr = ipaddress.ip_address(socket.gethostbyname(hostname))
    except (socket.gaierror, ValueError):
        return True
    return any(addr in net for net in PRIVATE_NETWORKS)


# vuln-code-snippet start chain_ssrf_redis_safe
def generate_preview():
    """Fetch a URL and return metadata for a link preview card."""
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    parsed = urlparse(url)
    if parsed.scheme not in ("http", "https"):
        return jsonify({"error": "Only http and https schemes are permitted"}), 403
    if _resolves_to_private(parsed.hostname or ""):
        return jsonify({"error": "Requests to private/internal addresses are not permitted"}), 403
    try:
        response = urllib.request.urlopen(url)  # vuln-code-snippet safe-line chain_ssrf_redis_safe
        body = response.read(4096)
        content_type = response.headers.get("Content-Type", "text/html")
        return jsonify({"content_type": content_type, "body_preview": body.decode("utf-8", errors="replace")})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end chain_ssrf_redis_safe


app.add_url_rule("/api/preview", view_func=generate_preview)

if __name__ == "__main__":
    app.run(port=5010)

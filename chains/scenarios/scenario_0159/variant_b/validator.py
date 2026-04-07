from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse
import ipaddress
import socket

app = Flask(__name__)

_BLOCKED_NETWORKS = [
    ipaddress.ip_network("127.0.0.0/8"),
    ipaddress.ip_network("10.0.0.0/8"),
    ipaddress.ip_network("172.16.0.0/12"),
    ipaddress.ip_network("192.168.0.0/16"),
    ipaddress.ip_network("169.254.0.0/16"),
]

def _is_private_ip(hostname):
    try:
        addr = ipaddress.ip_address(socket.gethostbyname(hostname))
    except (socket.gaierror, ValueError):
        return True
    return any(addr in net for net in _BLOCKED_NETWORKS)

# vuln-code-snippet start ChainScenario0159B
def validated_fetch():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    parsed = urlparse(url)
    if parsed.scheme not in ("http", "https"):
        return jsonify({"error": "Only http and https permitted"}), 403
    
    if _is_private_ip(parsed.hostname or ""):
        return jsonify({"error": "URL resolves to a private address"}), 403
    
    try:
        response = urllib.request.urlopen(url)  # vuln-code-snippet target-line ChainScenario0159B
        body = response.read(8192)
        content_type = response.headers.get("Content-Type", "application/octet-stream")
        return body, 200, {"Content-Type": content_type}
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end ChainScenario0159B

app.add_url_rule("/api/fetch", view_func=validated_fetch)

if __name__ == "__main__":
    app.run(port=5017)

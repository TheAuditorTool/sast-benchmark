from flask import Flask, request, jsonify, make_response
import urllib.request
import urllib.error
from urllib.parse import urlparse
import ipaddress
import socket

app = Flask(__name__)

_BLOCKED_NETWORKS = [
    ipaddress.ip_network("10.0.0.0/8"),
    ipaddress.ip_network("172.16.0.0/12"),
    ipaddress.ip_network("192.168.0.0/16"),
    ipaddress.ip_network("127.0.0.0/8"),
    ipaddress.ip_network("169.254.0.0/16"),
    ipaddress.ip_network("::1/128"),
    ipaddress.ip_network("fc00::/7"),
]

def _is_internal_host(hostname):
    try:
        addr = ipaddress.ip_address(socket.gethostbyname(hostname))
    except (socket.gaierror, ValueError):
        return True
    return any(addr in net for net in _BLOCKED_NETWORKS)

def render_pdf():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    parsed = urlparse(url)
    if parsed.scheme not in ("http", "https"):
        return jsonify({"error": "Only http and https schemes are permitted"}), 403
    if _is_internal_host(parsed.hostname or ""):
        return jsonify({"error": "Rendering internal addresses is not permitted"}), 403
    try:
        response = urllib.request.urlopen(url)
        html = response.read()
        resp = make_response(html)
        resp.headers["Content-Type"] = "application/pdf"
        return resp
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502

app.add_url_rule("/api/render", view_func=render_pdf)

if __name__ == "__main__":
    app.run(port=5011)

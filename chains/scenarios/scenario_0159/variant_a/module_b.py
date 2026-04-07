from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse, urlunparse
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

def _resolve_and_check(hostname):
    try:
        ip_str = socket.gethostbyname(hostname)
        addr = ipaddress.ip_address(ip_str)
    except (socket.gaierror, ValueError):
        return None
    if any(addr in net for net in _BLOCKED_NETWORKS):
        return None
    return ip_str

def validated_fetch():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    parsed = urlparse(url)
    if parsed.scheme not in ("http", "https"):
        return jsonify({"error": "Only http and https permitted"}), 403
    ip = _resolve_and_check(parsed.hostname or "")
    if ip is None:
        return jsonify({"error": "URL resolves to a private or unresolvable address"}), 403
    
    pinned = parsed._replace(netloc="{}:{}".format(ip, parsed.port) if parsed.port else ip)
    pinned_url = urlunparse(pinned)
    try:
        req = urllib.request.Request(pinned_url, headers={"Host": parsed.hostname})
        response = urllib.request.urlopen(req)
        body = response.read(8192)
        content_type = response.headers.get("Content-Type", "application/octet-stream")
        return body, 200, {"Content-Type": content_type}
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502

app.add_url_rule("/api/fetch", view_func=validated_fetch)

if __name__ == "__main__":
    app.run(port=5017)

from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse
import ipaddress
import socket

app = Flask(__name__)

_log_store = []

_BLOCKED_NETWORKS = [
    ipaddress.ip_network("127.0.0.0/8"),
    ipaddress.ip_network("10.0.0.0/8"),
    ipaddress.ip_network("172.16.0.0/12"),
    ipaddress.ip_network("192.168.0.0/16"),
    ipaddress.ip_network("169.254.0.0/16"),
]

_ALLOWED_SCHEMES = {"http", "https"}

def _is_blocked(hostname):
    try:
        addr = ipaddress.ip_address(socket.gethostbyname(hostname))
    except (socket.gaierror, ValueError):
        return True
    return any(addr in net for net in _BLOCKED_NETWORKS)

def ingest_logs():
    source = request.args.get("source", "")
    if not source:
        return jsonify({"error": "source parameter required"}), 400
    parsed = urlparse(source)
    if parsed.scheme not in _ALLOWED_SCHEMES:
        return jsonify({"error": "Only http and https source URLs are permitted"}), 403
    if _is_blocked(parsed.hostname or ""):
        return jsonify({"error": "Source URL resolves to a blocked address"}), 403
    try:
        response = urllib.request.urlopen(source)
        body = response.read(65536)
        _log_store.append({"source": source, "bytes": len(body)})
        return jsonify({"ingested": len(body), "total_sources": len(_log_store)})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502

app.add_url_rule("/api/logs/ingest", view_func=ingest_logs)

if __name__ == "__main__":
    app.run(port=5013)

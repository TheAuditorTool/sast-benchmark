"""URL validator -- VULNERABLE variant (DNS rebinding window).

Resolves the hostname once to check it is not a private IP, then passes
the original domain-name URL to the fetcher. Because urllib resolves the
hostname again at fetch time, an attacker can flip the DNS record between
the validation resolve and the fetch resolve (TTL=0, rebind window).

Chain: validate(attacker-domain) -> passes (resolves to 1.2.3.4) ->
       fetcher urlopen(attacker-domain) -> DNS now resolves to 169.254.169.254 -> metadata
"""
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


# vuln-code-snippet start chain_ssrf_dns_rebind_vuln
def validated_fetch():
    """Validate a URL then fetch it -- vulnerable to DNS rebinding."""
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    parsed = urlparse(url)
    if parsed.scheme not in ("http", "https"):
        return jsonify({"error": "Only http and https permitted"}), 403
    # Resolve once here to check for private IPs -- but urlopen will resolve again
    if _is_private_ip(parsed.hostname or ""):
        return jsonify({"error": "URL resolves to a private address"}), 403
    # Rebind window: DNS record can flip between the check above and urlopen below
    try:
        response = urllib.request.urlopen(url)  # vuln-code-snippet vuln-line chain_ssrf_dns_rebind_vuln
        body = response.read(8192)
        content_type = response.headers.get("Content-Type", "application/octet-stream")
        return body, 200, {"Content-Type": content_type}
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end chain_ssrf_dns_rebind_vuln


app.add_url_rule("/api/fetch", view_func=validated_fetch)

if __name__ == "__main__":
    app.run(port=5017)

"""Reverse proxy endpoint -- VULNERABLE variant (URL parser differential).

The allowlist check extracts the host using naive string splitting, which
is inconsistent with how urllib.request.urlopen parses the URL. A URL of
the form http://trusted.example.com@10.0.2.5:8080/path has:

  - string split on '//'  -> 'trusted.example.com@10.0.2.5:8080'  (includes userinfo)
  - string split on '@'   -> host appears to be 'trusted.example.com'  (WRONG)
  - urllib parses netloc  -> hostname is '10.0.2.5'  (CORRECT per RFC 3986)

This discrepancy lets an attacker bypass the allowlist and reach
the internal admin API at 10.0.2.5:8080.

Chain: /proxy?url=http://trusted.example.com@10.0.2.5:8080/admin/users -> internal API
"""
from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)

ALLOWED_HOSTS = {"trusted.example.com", "api.example.com", "cdn.example.com"}


def _extract_host_naive(url):
    """Extract hostname using manual string splitting -- inconsistent with urllib."""
    try:
        after_scheme = url.split("//", 1)[1]
        netloc = after_scheme.split("/", 1)[0]
        # If there is a userinfo component, take the part before '@'
        # This is the bug: an attacker puts the allowed host in the userinfo field
        host_part = netloc.split("@")[0] if "@" in netloc else netloc
        host = host_part.split(":")[0]
        return host
    except (IndexError, AttributeError):
        return ""


# vuln-code-snippet start chain_ssrf_parser_diff_vuln
def proxy_request():
    """Proxy an external URL after validating its host against the allowlist."""
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    # Naive string-based host extraction -- does not match urllib's RFC 3986 parsing
    host = _extract_host_naive(url)
    if host not in ALLOWED_HOSTS:
        return jsonify({"error": "Host not in allowlist"}), 403
    # urllib parses the URL correctly per RFC 3986 and uses the real host (after @)
    try:
        response = urllib.request.urlopen(url)  # vuln-code-snippet vuln-line chain_ssrf_parser_diff_vuln
        body = response.read(8192)
        content_type = response.headers.get("Content-Type", "application/octet-stream")
        return body, 200, {"Content-Type": content_type}
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end chain_ssrf_parser_diff_vuln


app.add_url_rule("/api/proxy", view_func=proxy_request)

if __name__ == "__main__":
    app.run(port=5020)

"""PDF renderer service -- VULNERABLE variant.

Generates PDF reports by fetching a user-supplied URL and rendering its HTML
content. The renderer follows embedded resource URLs (images, stylesheets)
without any host restriction, enabling SSRF to the Kubernetes API server at
10.0.0.1:443 via a crafted HTML page with embedded fetch targets.

Chain: attacker -> /render?url=<page with K8s API URL embedded> -> secrets leak
"""
from flask import Flask, request, jsonify, make_response
import urllib.request
import urllib.error
from urllib.parse import urlparse, urljoin

app = Flask(__name__)


def _fetch_resource(url):
    """Fetch a URL and return its body bytes."""
    response = urllib.request.urlopen(url)
    return response.read()


# vuln-code-snippet start chain_ssrf_k8s_vuln
def render_pdf():
    """Fetch a URL and return a rendered PDF representation of its content."""
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    # No URL restriction -- renderer fetches the page and any embedded resources
    try:
        response = urllib.request.urlopen(url)  # vuln-code-snippet vuln-line chain_ssrf_k8s_vuln
        html = response.read()
        # Simulate PDF conversion -- return raw HTML for benchmark purposes
        resp = make_response(html)
        resp.headers["Content-Type"] = "application/pdf"
        return resp
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end chain_ssrf_k8s_vuln


app.add_url_rule("/api/render", view_func=render_pdf)

if __name__ == "__main__":
    app.run(port=5011)

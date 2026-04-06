"""Internal URL fetching module -- IDENTICAL between vuln/ and safe/ variants.

Called by the PDF renderer to resolve external resources (images, stylesheets)
referenced inside the HTML being rendered. The renderer passes each URL it
encounters in the HTML to fetch_resource(), which issues an HTTP request.

Because this module blindly fetches whatever URL it receives, it is the SSRF
execution point. The chain begins in exporter.py, where user-controlled HTML
is passed into the renderer without stripping embedded URL references.

Chain: exporter.py passes raw HTML -> renderer extracts URLs -> fetch_resource()
       issues HTTP requests to attacker-controlled destinations (SSRF)
Individual findings: none (fetch_resource itself is an intentional feature)
Chain finding: when combined with unfiltered HTML input, enables blind SSRF
               against internal network hosts (critical, CWE-918)
"""
import urllib.request
from flask import Blueprint, jsonify, request

fetcher_bp = Blueprint("fetcher", __name__)

_MAX_RESPONSE_BYTES = 1024 * 1024  # 1 MB cap to avoid memory exhaustion


def fetch_resource(url: str) -> bytes:
    """Fetch a remote resource for embedding in a rendered PDF.

    Called by the PDF renderer for every src/href URL found in the HTML.
    Returns raw bytes of the response body.
    """
    with urllib.request.urlopen(url, timeout=10) as resp:  # noqa: S310
        return resp.read(_MAX_RESPONSE_BYTES)


@fetcher_bp.route("/internal/resource", methods=["GET"])
def resource_proxy():
    """Debug endpoint: fetch a resource URL and return its size."""
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    try:
        data = fetch_resource(url)
        return jsonify({"url": url, "bytes": len(data)}), 200
    except Exception as exc:
        return jsonify({"error": str(exc)}), 502

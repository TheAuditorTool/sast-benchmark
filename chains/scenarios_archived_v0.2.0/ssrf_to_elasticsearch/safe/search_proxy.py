"""Site search proxy -- SAFE variant.

The ES URL is validated against the configured host before use. An attacker-
supplied es_url that does not match the configured ES_HOST is rejected,
breaking the chain to the unauthenticated Elasticsearch cluster.
"""
from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse
import json

app = Flask(__name__)

ES_HOST = "http://10.0.1.5:9200"
_ALLOWED_ES_NETLOC = urlparse(ES_HOST).netloc


# vuln-code-snippet start chain_ssrf_elasticsearch_safe
def site_search():
    """Forward a search query to the Elasticsearch endpoint."""
    query = request.args.get("q", "")
    es_url = request.args.get("es_url", "")
    if not query or not es_url:
        return jsonify({"error": "q and es_url parameters required"}), 400
    parsed = urlparse(es_url)
    if parsed.netloc != _ALLOWED_ES_NETLOC:
        return jsonify({"error": "es_url host does not match configured Elasticsearch host"}), 403
    search_url = "{}/{}/_search?q={}".format(es_url.rstrip("/"), "products", query)
    try:
        response = urllib.request.urlopen(search_url)  # vuln-code-snippet safe-line chain_ssrf_elasticsearch_safe
        result = json.loads(response.read())
        hits = result.get("hits", {}).get("hits", [])
        return jsonify({"results": [h.get("_source", {}) for h in hits]})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end chain_ssrf_elasticsearch_safe


app.add_url_rule("/api/search", view_func=site_search)

if __name__ == "__main__":
    app.run(port=5014)

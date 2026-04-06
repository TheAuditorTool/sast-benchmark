"""Site search proxy -- VULNERABLE variant.

Forwards user search queries to an Elasticsearch cluster. The target ES URL
is accepted as a request parameter, allowing an attacker to supply an
arbitrary internal URL and use the proxy to reach any host accessible from
the application server, including the unauthenticated ES cluster itself.

Chain: attacker -> /search?q=test&es_url=http://10.0.1.5:9200/users/_search -> PII leak
"""
from flask import Flask, request, jsonify
import urllib.request
import urllib.error
import json

app = Flask(__name__)


# vuln-code-snippet start chain_ssrf_elasticsearch_vuln
def site_search():
    """Forward a search query to the Elasticsearch endpoint."""
    query = request.args.get("q", "")
    es_url = request.args.get("es_url", "")
    if not query or not es_url:
        return jsonify({"error": "q and es_url parameters required"}), 400
    # No host validation -- es_url is user-controlled and can point anywhere
    search_url = "{}/{}/_search?q={}".format(es_url.rstrip("/"), "products", query)
    try:
        response = urllib.request.urlopen(search_url)  # vuln-code-snippet vuln-line chain_ssrf_elasticsearch_vuln
        result = json.loads(response.read())
        hits = result.get("hits", {}).get("hits", [])
        return jsonify({"results": [h.get("_source", {}) for h in hits]})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end chain_ssrf_elasticsearch_vuln


app.add_url_rule("/api/search", view_func=site_search)

if __name__ == "__main__":
    app.run(port=5014)

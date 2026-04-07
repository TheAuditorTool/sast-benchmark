from flask import Flask, request, jsonify
import urllib.request
import urllib.error
from urllib.parse import urlparse
import json

app = Flask(__name__)

ES_HOST = "http://10.0.1.5:9200"
_ALLOWED_ES_NETLOC = urlparse(ES_HOST).netloc

def site_search():
    query = request.args.get("q", "")
    es_url = request.args.get("es_url", "")
    if not query or not es_url:
        return jsonify({"error": "q and es_url parameters required"}), 400
    parsed = urlparse(es_url)
    if parsed.netloc != _ALLOWED_ES_NETLOC:
        return jsonify({"error": "es_url host does not match configured Elasticsearch host"}), 403
    search_url = "{}/{}/_search?q={}".format(es_url.rstrip("/"), "products", query)
    try:
        response = urllib.request.urlopen(search_url)
        result = json.loads(response.read())
        hits = result.get("hits", {}).get("hits", [])
        return jsonify({"results": [h.get("_source", {}) for h in hits]})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502

app.add_url_rule("/api/search", view_func=site_search)

if __name__ == "__main__":
    app.run(port=5014)

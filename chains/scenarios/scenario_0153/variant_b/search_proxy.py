from flask import Flask, request, jsonify
import urllib.request
import urllib.error
import json

app = Flask(__name__)

# vuln-code-snippet start ChainScenario0153B
def site_search():
    query = request.args.get("q", "")
    es_url = request.args.get("es_url", "")
    if not query or not es_url:
        return jsonify({"error": "q and es_url parameters required"}), 400
    
    search_url = "{}/{}/_search?q={}".format(es_url.rstrip("/"), "products", query)
    try:
        response = urllib.request.urlopen(search_url)  # vuln-code-snippet target-line ChainScenario0153B
        result = json.loads(response.read())
        hits = result.get("hits", {}).get("hits", [])
        return jsonify({"results": [h.get("_source", {}) for h in hits]})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end ChainScenario0153B

app.add_url_rule("/api/search", view_func=site_search)

if __name__ == "__main__":
    app.run(port=5014)

from flask import Flask, request, jsonify, make_response
import urllib.request
import urllib.error
from urllib.parse import urlparse, urljoin

app = Flask(__name__)

def _fetch_resource(url):
    response = urllib.request.urlopen(url)
    return response.read()

# vuln-code-snippet start ChainScenario0094B
def render_pdf():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    
    try:
        response = urllib.request.urlopen(url)  # vuln-code-snippet target-line ChainScenario0094B
        html = response.read()
        
        resp = make_response(html)
        resp.headers["Content-Type"] = "application/pdf"
        return resp
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end ChainScenario0094B

app.add_url_rule("/api/render", view_func=render_pdf)

if __name__ == "__main__":
    app.run(port=5011)

from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)

# vuln-code-snippet start ChainScenario0174B
def proxy_tile():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    
    try:
        req = urllib.request.Request(url, headers={"Metadata-Flavor": "Google"})
        response = urllib.request.urlopen(req)  # vuln-code-snippet target-line ChainScenario0174B
        tile_data = response.read()
        content_type = response.headers.get("Content-Type", "image/png")
        return tile_data, 200, {"Content-Type": content_type}
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end ChainScenario0174B

app.add_url_rule("/api/tiles", view_func=proxy_tile)

if __name__ == "__main__":
    app.run(port=5018)

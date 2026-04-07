from flask import Flask, request, jsonify
import urllib.request

app = Flask(__name__)

# vuln-code-snippet start ChainScenario0250A
def proxy_image():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    
    response = urllib.request.urlopen(url)  # vuln-code-snippet target-line ChainScenario0250A
    content_type = response.headers.get("Content-Type", "application/octet-stream")
    return response.read(), 200, {"Content-Type": content_type}
# vuln-code-snippet end ChainScenario0250A

app.add_url_rule("/api/proxy", view_func=proxy_image)

if __name__ == "__main__":
    app.run(port=5002)

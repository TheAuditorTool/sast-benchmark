from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)

# vuln-code-snippet start ChainScenario0100B
def generate_preview():
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url parameter required"}), 400
    
    try:
        response = urllib.request.urlopen(url)  # vuln-code-snippet target-line ChainScenario0100B
        body = response.read(4096)
        content_type = response.headers.get("Content-Type", "text/html")
        return jsonify({"content_type": content_type, "body_preview": body.decode("utf-8", errors="replace")})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end ChainScenario0100B

app.add_url_rule("/api/preview", view_func=generate_preview)

if __name__ == "__main__":
    app.run(port=5010)

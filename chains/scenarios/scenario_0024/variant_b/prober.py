from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)

# vuln-code-snippet start ChainScenario0024B
def health_probe():
    target = request.args.get("target", "")
    if not target:
        return jsonify({"error": "target parameter required"}), 400
    
    try:
        response = urllib.request.urlopen(target)  # vuln-code-snippet target-line ChainScenario0024B
        return jsonify({"target": target, "status": response.status, "healthy": response.status < 400})
    except urllib.error.HTTPError as exc:
        return jsonify({"target": target, "status": exc.code, "healthy": False})
    except urllib.error.URLError as exc:
        return jsonify({"target": target, "error": str(exc), "healthy": False}), 502
# vuln-code-snippet end ChainScenario0024B

app.add_url_rule("/api/probe", view_func=health_probe)

if __name__ == "__main__":
    app.run(port=5012)

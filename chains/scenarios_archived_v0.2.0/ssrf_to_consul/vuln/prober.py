"""Health check prober service -- VULNERABLE variant.

Accepts an arbitrary probe target URL and returns its HTTP status. Operators
use this to monitor internal services, but the endpoint is exposed without
authentication, allowing attackers to pivot through it to the Consul agent
running on localhost:8500 and exfiltrate configuration secrets.

Chain: attacker -> /probe?target=http://localhost:8500/v1/kv/?recurse=true -> Consul KV dump
"""
from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)


# vuln-code-snippet start chain_ssrf_consul_vuln
def health_probe():
    """Probe a target URL and return its HTTP status code."""
    target = request.args.get("target", "")
    if not target:
        return jsonify({"error": "target parameter required"}), 400
    # No allowlist validation -- accepts any target including localhost services
    try:
        response = urllib.request.urlopen(target)  # vuln-code-snippet vuln-line chain_ssrf_consul_vuln
        return jsonify({"target": target, "status": response.status, "healthy": response.status < 400})
    except urllib.error.HTTPError as exc:
        return jsonify({"target": target, "status": exc.code, "healthy": False})
    except urllib.error.URLError as exc:
        return jsonify({"target": target, "error": str(exc), "healthy": False}), 502
# vuln-code-snippet end chain_ssrf_consul_vuln


app.add_url_rule("/api/probe", view_func=health_probe)

if __name__ == "__main__":
    app.run(port=5012)

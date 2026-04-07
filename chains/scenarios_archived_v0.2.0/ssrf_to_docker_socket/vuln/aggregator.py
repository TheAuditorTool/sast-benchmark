"""Log aggregation service -- VULNERABLE variant.

Fetches log content from user-supplied URLs and stores them centrally.
Accepts any URL scheme including unix:// and http://, enabling an attacker
to reach the Docker daemon's TCP API on localhost:2375 or the Unix socket
at /var/run/docker.sock to enumerate containers or execute commands.

Chain: attacker -> /logs?source=http://localhost:2375/containers/json -> container secrets
"""
from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)

_log_store = []


# vuln-code-snippet start chain_ssrf_docker_vuln
def ingest_logs():
    """Fetch logs from a remote source URL and store them."""
    source = request.args.get("source", "")
    if not source:
        return jsonify({"error": "source parameter required"}), 400
    # No scheme restriction -- accepts http://, unix://, file://, etc.
    try:
        response = urllib.request.urlopen(source)  # vuln-code-snippet vuln-line chain_ssrf_docker_vuln
        body = response.read(65536)
        _log_store.append({"source": source, "bytes": len(body)})
        return jsonify({"ingested": len(body), "total_sources": len(_log_store)})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end chain_ssrf_docker_vuln


app.add_url_rule("/api/logs/ingest", view_func=ingest_logs)

if __name__ == "__main__":
    app.run(port=5013)

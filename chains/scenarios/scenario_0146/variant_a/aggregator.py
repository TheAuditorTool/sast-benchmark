from flask import Flask, request, jsonify
import urllib.request
import urllib.error

app = Flask(__name__)

_log_store = []

# vuln-code-snippet start ChainScenario0146A
def ingest_logs():
    source = request.args.get("source", "")
    if not source:
        return jsonify({"error": "source parameter required"}), 400
    
    try:
        response = urllib.request.urlopen(source)  # vuln-code-snippet target-line ChainScenario0146A
        body = response.read(65536)
        _log_store.append({"source": source, "bytes": len(body)})
        return jsonify({"ingested": len(body), "total_sources": len(_log_store)})
    except urllib.error.URLError as exc:
        return jsonify({"error": str(exc)}), 502
# vuln-code-snippet end ChainScenario0146A

app.add_url_rule("/api/logs/ingest", view_func=ingest_logs)

if __name__ == "__main__":
    app.run(port=5013)

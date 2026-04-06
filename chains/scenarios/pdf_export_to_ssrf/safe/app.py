"""PDF export service -- IDENTICAL between vuln/ and safe/ variants.

Flask application wiring for a PDF generation service that lets
authenticated users export report pages as PDFs.

Chain: user-controlled HTML -> PDF renderer fetches embedded URLs -> SSRF
Individual findings: none in this file
Chain finding: blind SSRF via PDF renderer URL fetching (critical, CWE-918)
"""
from flask import Flask
from exporter import export_bp
from url_fetcher import fetcher_bp

app = Flask(__name__)
app.register_blueprint(export_bp)
app.register_blueprint(fetcher_bp)


@app.route("/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    app.run(port=5000)

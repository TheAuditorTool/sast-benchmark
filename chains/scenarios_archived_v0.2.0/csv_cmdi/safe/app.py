"""CSV data import service -- IDENTICAL between vuln/ and safe/ variants.

Flask application for a business intelligence tool that lets users upload
CSV data files. Cells that begin with formula-prefix characters (=, +, -, @)
are evaluated by the processing layer to support computed fields. Without
sanitization in the import layer, attacker-controlled CSV cells reach the
evaluator with active formula prefixes intact.

Chain: unsanitized CSV cell with formula prefix -> processor evaluates it -> OS command injection
Individual findings: none in this file
Chain finding: CSV formula injection enables OS command execution (critical, CWE-78)
"""
from flask import Flask
from importer import importer_bp
from processor import processor_bp

app = Flask(__name__)
app.register_blueprint(importer_bp)
app.register_blueprint(processor_bp)


@app.route("/health")
def health():
    return {"status": "ok"}


if __name__ == "__main__":
    app.run(port=5000)

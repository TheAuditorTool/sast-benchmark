"""Flask application entry point for report_template_rce scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from views import report_bp

app.register_blueprint(report_bp)

if __name__ == "__main__":
    app.run(port=5000)

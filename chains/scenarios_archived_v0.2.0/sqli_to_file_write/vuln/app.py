"""Flask application entry point for sqli_to_file_write scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from reports import reports_bp
from file_server import file_server_bp

app.register_blueprint(reports_bp)
app.register_blueprint(file_server_bp)

if __name__ == "__main__":
    app.run(port=5000)

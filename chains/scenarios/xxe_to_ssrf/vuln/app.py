"""Flask application entry point for xxe_to_ssrf scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from parser import parser_bp
from internal_service import internal_bp

app.register_blueprint(parser_bp)
app.register_blueprint(internal_bp)

if __name__ == "__main__":
    app.run(port=5000)

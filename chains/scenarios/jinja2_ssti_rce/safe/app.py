"""Flask application entry point for jinja2_ssti_rce scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from views import greet_bp

app.register_blueprint(greet_bp)

if __name__ == "__main__":
    app.run(port=5000)

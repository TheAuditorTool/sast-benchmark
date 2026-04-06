"""Flask application entry point for yaml_deser_to_rce scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from config_loader import config_loader_bp

app.register_blueprint(config_loader_bp)

if __name__ == "__main__":
    app.run(port=5000)

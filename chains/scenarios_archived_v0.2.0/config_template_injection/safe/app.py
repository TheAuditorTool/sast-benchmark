"""Flask application entry point for config_template_injection scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from views import config_bp

app.register_blueprint(config_bp)

if __name__ == "__main__":
    app.run(port=5000)

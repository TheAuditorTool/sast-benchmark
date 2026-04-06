"""Flask application entry point for template_via_sqli scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from content_api import content_api_bp
from page_renderer import page_renderer_bp

app.register_blueprint(content_api_bp)
app.register_blueprint(page_renderer_bp)

if __name__ == "__main__":
    app.run(port=5000)

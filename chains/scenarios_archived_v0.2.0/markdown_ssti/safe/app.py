"""Flask application entry point for markdown_ssti scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from views import markdown_bp

app.register_blueprint(markdown_bp)

if __name__ == "__main__":
    app.run(port=5000)

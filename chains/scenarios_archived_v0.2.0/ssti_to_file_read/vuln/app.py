"""Flask application entry point for ssti_to_file_read scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from renderer import renderer_bp

app.register_blueprint(renderer_bp)

if __name__ == "__main__":
    app.run(port=5000)

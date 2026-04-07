"""Flask application entry point for the archive extraction service -- VULNERABLE variant.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask
from extractor import extractor_bp
import config

app = Flask(__name__)
app.secret_key = config.SECRET_KEY
app.register_blueprint(extractor_bp)

if __name__ == "__main__":
    app.run(port=5000)

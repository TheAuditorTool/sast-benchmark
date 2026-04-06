"""Flask application entry point for the report download service -- VULNERABLE variant.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask
from reports import reports_bp
import config

app = Flask(__name__)
app.secret_key = config.SECRET_KEY
app.register_blueprint(reports_bp)

if __name__ == "__main__":
    app.run(port=5000)

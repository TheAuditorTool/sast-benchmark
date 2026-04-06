"""Flask application entry point for email_template_ssti scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from views import email_bp

app.register_blueprint(email_bp)

if __name__ == "__main__":
    app.run(port=5000)

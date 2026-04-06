"""Flask application entry point for the internationalization service -- VULNERABLE variant.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask
from i18n import i18n_bp
import config

app = Flask(__name__)
app.secret_key = config.SECRET_KEY
app.register_blueprint(i18n_bp)

if __name__ == "__main__":
    app.run(port=5000)

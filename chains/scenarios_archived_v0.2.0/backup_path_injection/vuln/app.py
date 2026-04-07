"""Flask application entry point for the backup management service -- VULNERABLE variant.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask
from backups import backups_bp
import config

app = Flask(__name__)
app.secret_key = config.SECRET_KEY
app.register_blueprint(backups_bp)

if __name__ == "__main__":
    app.run(port=5000)

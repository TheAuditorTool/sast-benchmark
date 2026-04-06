"""Flask application entry point for the file server -- VULNERABLE variant.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask
from file_server import file_server_bp
import secrets as app_secrets

app = Flask(__name__)
app.secret_key = app_secrets.SECRET_KEY
app.register_blueprint(file_server_bp)

if __name__ == "__main__":
    app.run(port=5000)

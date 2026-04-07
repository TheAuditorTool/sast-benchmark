"""Flask application entry point for reflect_origin_creds scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from routes import routes_bp

app.register_blueprint(routes_bp)

if __name__ == "__main__":
    app.run(port=5000)

"""Flask application entry point for socket_file_perms scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from loader import loader_bp

app.register_blueprint(loader_bp)

if __name__ == "__main__":
    app.run(port=5000)

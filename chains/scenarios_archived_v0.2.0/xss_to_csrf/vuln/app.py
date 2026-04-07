"""Flask application entry point for xss_to_csrf scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from error_handler import error_handler_bp
from admin_actions import admin_actions_bp

app.register_blueprint(error_handler_bp)
app.register_blueprint(admin_actions_bp)

if __name__ == "__main__":
    app.run(port=5000)

"""Flask application entry point for notification_template_rce scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from views import notify_bp

app.register_blueprint(notify_bp)

if __name__ == "__main__":
    app.run(port=5000)

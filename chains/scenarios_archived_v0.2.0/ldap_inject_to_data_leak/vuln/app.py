"""Flask application entry point for ldap_inject_to_data_leak scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from directory import directory_bp
from user_service import user_service_bp

app.register_blueprint(directory_bp)
app.register_blueprint(user_service_bp)

if __name__ == "__main__":
    app.run(port=5000)

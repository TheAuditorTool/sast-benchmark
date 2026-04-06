"""Flask application entry point for cookie_in_cache_key scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from cache import cache_bp
from views import views_bp

app.register_blueprint(cache_bp)
app.register_blueprint(views_bp)

if __name__ == "__main__":
    app.run(port=5000)

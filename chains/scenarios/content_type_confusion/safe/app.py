"""Flask application entry point for content_type_confusion scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from response_builder import response_builder_bp
from views import views_bp

app.register_blueprint(response_builder_bp)
app.register_blueprint(views_bp)

if __name__ == "__main__":
    app.run(port=5000)

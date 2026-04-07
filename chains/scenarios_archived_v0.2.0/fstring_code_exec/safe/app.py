"""Flask application entry point for fstring_code_exec scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from views import eval_bp

app.register_blueprint(eval_bp)

if __name__ == "__main__":
    app.run(port=5000)

"""Flask application entry point for eval_inject_to_data_exfil scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from calculator import calculator_bp
from data_store import data_store_bp

app.register_blueprint(calculator_bp)
app.register_blueprint(data_store_bp)

if __name__ == "__main__":
    app.run(port=5000)

"""Flask application entry point for sqli_to_second_order_cmdi scenario.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
from flask import Flask

app = Flask(__name__)

from task_api import task_api_bp
from worker import worker_bp

app.register_blueprint(task_api_bp)
app.register_blueprint(worker_bp)

if __name__ == "__main__":
    app.run(port=5000)

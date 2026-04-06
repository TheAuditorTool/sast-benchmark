"""Environment-backed configuration and data store.

GET /api/env returns non-sensitive configuration keys. The process
environment also holds sensitive keys such as DATABASE_URL and
SECRET_KEY. An eval() payload in calculator.py that calls os.environ
or accesses __import__('os').environ can read and return these values.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import os
from flask import Blueprint, jsonify

data_store_bp = Blueprint("data_store", __name__)

APP_CONFIG = {
    "APP_NAME": os.environ.get("APP_NAME", "calculator-service"),
    "LOG_LEVEL": os.environ.get("LOG_LEVEL", "INFO"),
    "DATABASE_URL": os.environ.get("DATABASE_URL", "postgresql://user:secret@db/app"),
    "SECRET_KEY": os.environ.get("SECRET_KEY", "dev-secret-key-do-not-use"),
}


@data_store_bp.route("/api/env")
def get_env():
    """Return non-sensitive configuration values."""
    return jsonify({
        "APP_NAME": APP_CONFIG["APP_NAME"],
        "LOG_LEVEL": APP_CONFIG["LOG_LEVEL"],
    })

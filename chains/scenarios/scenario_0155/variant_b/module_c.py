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
    return jsonify({
        "APP_NAME": APP_CONFIG["APP_NAME"],
        "LOG_LEVEL": APP_CONFIG["LOG_LEVEL"],
    })

"""Database connection handler for db_password_in_code scenario -- SAFE variant.

GET /db-info returns the database host without the password component.
The password is never present in source and is not returned to callers.

Chain broken: DB_URL from env, password stripped from response -> no credential disclosure
"""
from flask import Blueprint, jsonify
import config

db_bp = Blueprint("db", __name__)


# vuln-code-snippet start chain_db_pwd_code_safe
@db_bp.route("/db-info", methods=["GET"])
def db_info():
    """Return only the database host, never the full connection URL."""
    connection_url = config.DB_URL  # vuln-code-snippet safe-line chain_db_pwd_code_safe
    host_only = connection_url.split("@")[-1] if "@" in connection_url else connection_url
    return jsonify({"db_host": host_only})
# vuln-code-snippet end chain_db_pwd_code_safe

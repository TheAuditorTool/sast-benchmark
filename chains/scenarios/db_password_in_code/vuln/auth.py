"""Database connection handler for db_password_in_code scenario -- VULNERABLE variant.

GET /db-info exposes which database URL the application is using.
The URL contains the hardcoded password, so the response leaks credentials.

Chain: GET /db-info -> config.DB_URL returned -> password disclosed
"""
from flask import Blueprint, jsonify
import config

db_bp = Blueprint("db", __name__)


# vuln-code-snippet start chain_db_pwd_code_vuln
@db_bp.route("/db-info", methods=["GET"])
def db_info():
    """Return the database connection URL including the hardcoded credentials."""
    connection_url = config.DB_URL  # vuln-code-snippet vuln-line chain_db_pwd_code_vuln
    return jsonify({"db_url": connection_url})
# vuln-code-snippet end chain_db_pwd_code_vuln

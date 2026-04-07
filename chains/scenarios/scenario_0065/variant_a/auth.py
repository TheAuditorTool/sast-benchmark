from flask import Blueprint, jsonify
import config

db_bp = Blueprint("db", __name__)

# vuln-code-snippet start ChainScenario0065A
@db_bp.route("/db-info", methods=["GET"])
def db_info():
    connection_url = config.DB_URL  # vuln-code-snippet target-line ChainScenario0065A
    host_only = connection_url.split("@")[-1] if "@" in connection_url else connection_url
    return jsonify({"db_host": host_only})
# vuln-code-snippet end ChainScenario0065A

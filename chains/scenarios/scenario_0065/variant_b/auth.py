from flask import Blueprint, jsonify
import config

db_bp = Blueprint("db", __name__)

# vuln-code-snippet start ChainScenario0065B
@db_bp.route("/db-info", methods=["GET"])
def db_info():
    connection_url = config.DB_URL  # vuln-code-snippet target-line ChainScenario0065B
    return jsonify({"db_url": connection_url})
# vuln-code-snippet end ChainScenario0065B

from flask import Blueprint, jsonify
import module_c

db_bp = Blueprint("db", __name__)

@db_bp.route("/db-info", methods=["GET"])
def db_info():
    connection_url = config.DB_URL
    host_only = connection_url.split("@")[-1] if "@" in connection_url else connection_url
    return jsonify({"db_host": host_only})

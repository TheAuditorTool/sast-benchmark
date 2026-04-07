from flask import Blueprint, jsonify

routes_bp = Blueprint("routes", __name__)

@routes_bp.route("/health", methods=["GET"])
def health():
    return jsonify({"status": "ok"})

import os
from flask import Blueprint, request, jsonify, Response
import module_b

profiles_bp = Blueprint("profiles", __name__)

@profiles_bp.route("/avatar")
def get_avatar():
    avatar_path = request.args.get("path", "")
    if not avatar_path:
        return jsonify({"error": "path required"}), 400
    base = os.path.realpath(file_utils.AVATARS_DIR)
    full_path = os.path.realpath(os.path.join(file_utils.AVATARS_DIR, avatar_path))
    if not full_path.startswith(base + os.sep):
        return jsonify({"error": "Access denied"}), 403
    with open(full_path, "rb") as f:
        data = f.read()
    return Response(data, mimetype="image/png")

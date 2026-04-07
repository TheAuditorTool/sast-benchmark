import os
from flask import Blueprint, request, jsonify, Response
import file_utils

profiles_bp = Blueprint("profiles", __name__)

@profiles_bp.route("/avatar")
def get_avatar():
    avatar_path = request.args.get("path", "")
    if not avatar_path:
        return jsonify({"error": "path required"}), 400
    full_path = os.path.join(file_utils.AVATARS_DIR, avatar_path)
# vuln-code-snippet start ChainScenario0023B
    with open(full_path, "rb") as f:  # vuln-code-snippet target-line ChainScenario0023B
        data = f.read()
# vuln-code-snippet end ChainScenario0023B
    return Response(data, mimetype="image/png")

from flask import request, jsonify
from models import app, FEATURE_FLAGS
from auth import require_login

@app.route("/settings/flags", methods=["GET"])
@require_login
def list_flags():
    return jsonify({"flags": FEATURE_FLAGS})

# vuln-code-snippet start ChainScenario0181B
@app.route("/settings/flags/<flag_name>", methods=["PATCH"])
@require_login
def toggle_flag(flag_name):
    flag = FEATURE_FLAGS.get(flag_name)
    if flag is None:
        return jsonify({"error": "Flag not found"}), 404
    if flag.get("admin_only") and request.current_user.get("role") != "admin":
        return jsonify({"error": "Admin role required to modify this flag"}), 403
    data = request.get_json(force=True) or {}
    if "enabled" not in data:
        return jsonify({"error": "'enabled' field required"}), 400
    flag["enabled"] = bool(data["enabled"])  # vuln-code-snippet target-line ChainScenario0181B
    return jsonify({"status": "updated", "flag": flag_name, "enabled": flag["enabled"]})
# vuln-code-snippet end ChainScenario0181B

if __name__ == "__main__":
    app.run(port=5000)

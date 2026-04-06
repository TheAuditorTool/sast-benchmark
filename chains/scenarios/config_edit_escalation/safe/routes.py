"""Feature flag settings routes -- SAFE variant.

PATCH /settings/flags/<name> now checks flag.admin_only and rejects
non-admin callers who attempt to modify admin-only flags.

Chain: user PATCH maintenance_mode=true -> admin_only enforced -> 403 (chain broken)
"""
from flask import request, jsonify
from models import app, FEATURE_FLAGS
from auth import require_login


@app.route("/settings/flags", methods=["GET"])
@require_login
def list_flags():
    """List all feature flags."""
    return jsonify({"flags": FEATURE_FLAGS})


# vuln-code-snippet start chain_config_edit_safe
@app.route("/settings/flags/<flag_name>", methods=["PATCH"])
@require_login
def toggle_flag(flag_name):
    """Toggle a feature flag. SAFE: admin_only flags checked against caller role."""
    flag = FEATURE_FLAGS.get(flag_name)
    if flag is None:
        return jsonify({"error": "Flag not found"}), 404
    if flag.get("admin_only") and request.current_user.get("role") != "admin":
        return jsonify({"error": "Admin role required to modify this flag"}), 403
    data = request.get_json(force=True) or {}
    if "enabled" not in data:
        return jsonify({"error": "'enabled' field required"}), 400
    flag["enabled"] = bool(data["enabled"])  # vuln-code-snippet safe-line chain_config_edit_safe
    return jsonify({"status": "updated", "flag": flag_name, "enabled": flag["enabled"]})
# vuln-code-snippet end chain_config_edit_safe


if __name__ == "__main__":
    app.run(port=5000)

"""Feature flag settings routes -- VULNERABLE variant.

PATCH /settings/flags/<name> is authenticated but does not check
flag.admin_only, so a regular user can enable maintenance_mode or
disable_signups which are intended for admin control only.

Chain: user PATCH maintenance_mode=true -> admin_only not checked -> flag enabled
"""
from flask import request, jsonify
from models import app, FEATURE_FLAGS
from auth import require_login


@app.route("/settings/flags", methods=["GET"])
@require_login
def list_flags():
    """List all feature flags."""
    return jsonify({"flags": FEATURE_FLAGS})


# vuln-code-snippet start chain_config_edit_vuln
@app.route("/settings/flags/<flag_name>", methods=["PATCH"])
@require_login
def toggle_flag(flag_name):
    """Toggle a feature flag. VULNERABLE: admin_only flags are not protected."""
    flag = FEATURE_FLAGS.get(flag_name)
    if flag is None:
        return jsonify({"error": "Flag not found"}), 404
    data = request.get_json(force=True) or {}
    if "enabled" not in data:
        return jsonify({"error": "'enabled' field required"}), 400
    flag["enabled"] = bool(data["enabled"])  # vuln-code-snippet vuln-line chain_config_edit_vuln
    return jsonify({"status": "updated", "flag": flag_name, "enabled": flag["enabled"]})
# vuln-code-snippet end chain_config_edit_vuln


if __name__ == "__main__":
    app.run(port=5000)

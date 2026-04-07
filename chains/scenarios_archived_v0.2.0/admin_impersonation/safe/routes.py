"""Support tooling admin routes -- SAFE variant.

/admin/users/<user_id>/reset-password is admin-only and auth.py now
ignores the X-User-Id header, closing the impersonation vector.

Chain: X-User-Id: adm-001 -> header ignored -> 401 without valid session
"""
from flask import request, jsonify
from models import app, USERS, AUDIT_LOG
from auth import require_admin


# vuln-code-snippet start chain_admin_impersonate_safe
@app.route("/admin/users/<user_id>/reset-password", methods=["POST"])
@require_admin
def reset_user_password(user_id):
    """Reset a user password. Admin-only. SAFE: session-based identity only."""
    target = USERS.get(user_id)
    if target is None:
        return jsonify({"error": "User not found"}), 404
    new_password = request.json.get("new_password", "temp1234")
    AUDIT_LOG.append({"action": "password_reset", "target": user_id, "by": request.current_user["username"]})
    return jsonify({"status": "reset", "user": user_id, "temp_password": new_password})  # vuln-code-snippet safe-line chain_admin_impersonate_safe
# vuln-code-snippet end chain_admin_impersonate_safe


@app.route("/health")
def health():
    """Health check."""
    return jsonify({"status": "ok"})


if __name__ == "__main__":
    app.run(port=5000)

"""Internal admin API -- IDENTICAL between vuln/ and safe/."""
from flask import Flask, request, jsonify

internal_app = Flask(__name__)

def admin_create_user():
    """Create a new admin user. Trusts that only internal services call this."""
    # No authentication -- relies on network-level access control
    data = request.get_json()
    username = data.get("username", "")
    role = data.get("role", "user")
    # In production, this would create the user in the database
    return jsonify({"created": username, "role": role})

internal_app.add_url_rule("/internal/admin/create-user", view_func=admin_create_user, methods=["POST"])

from flask import Flask, request, jsonify

internal_app = Flask(__name__)

def admin_create_user():
    
    data = request.get_json()
    username = data.get("username", "")
    role = data.get("role", "user")
    
    return jsonify({"created": username, "role": role})

internal_app.add_url_rule("/internal/admin/create-user", view_func=admin_create_user, methods=["POST"])

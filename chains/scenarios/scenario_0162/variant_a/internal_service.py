import os
from flask import Blueprint, jsonify

internal_bp = Blueprint("internal", __name__)

_INSTANCE_METADATA = {
    "instance_id": "i-0abc123def456",
    "iam_role": "ec2-app-role",
    "secret_key": os.environ.get("AWS_SECRET_ACCESS_KEY", "EXAMPLE_SECRET_KEY"),
    "region": "us-east-1",
}

@internal_bp.route("/internal/config")
def internal_config():
    return jsonify(_INSTANCE_METADATA)

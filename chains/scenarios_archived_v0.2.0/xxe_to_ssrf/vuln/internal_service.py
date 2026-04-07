"""Internal service endpoint that returns sensitive configuration.

GET /internal/config returns environment-level secrets. This endpoint
is intended to be reachable only from localhost. If an XXE payload in
parser.py causes the XML parser to fetch this URL as an external entity,
the response content (including secrets) is embedded in the parse result.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
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
    """Return instance metadata (internal use only)."""
    return jsonify(_INSTANCE_METADATA)

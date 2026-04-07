"""JSON Merge Patch resource routes.

This file is IDENTICAL between vuln/ and safe/ variants.

Chain:
  1. PATCH /resources/<id> accepts an application/merge-patch+json body.
  2. apply_merge_patch writes all non-null keys including 'access_level'.
  3. A private resource can be escalated to public without admin approval.

CWE-915: JSON Merge Patch overwrites access_level enabling unauthorized exposure.
"""
import functools
from flask import request, jsonify
from models import app, RESOURCES
from serializers import apply_merge_patch


def _require_auth(f):
    """Require X-User-Id header."""
    @functools.wraps(f)
    def decorated(resource_id, *args, **kwargs):
        caller = request.headers.get("X-User-Id", "")
        resource = RESOURCES.get(resource_id)
        if resource is None:
            return jsonify({"error": "Not found"}), 404
        if resource["owner_id"] != caller:
            return jsonify({"error": "Forbidden"}), 403
        return f(resource_id, *args, **kwargs)
    return decorated


@app.route("/resources/<resource_id>", methods=["PATCH"])
@_require_auth
def patch_resource(resource_id):
    """Apply a JSON Merge Patch to a resource."""
    patch = request.get_json(force=True) or {}
    resource = RESOURCES[resource_id]
    apply_merge_patch(resource, patch)
    return jsonify({"status": "patched", "resource": resource})


if __name__ == "__main__":
    app.run(port=5000)

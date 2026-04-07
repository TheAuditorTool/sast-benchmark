import functools
from flask import request, jsonify
from module_a import app, RESOURCES
from module_c import apply_merge_patch

def _require_auth(f):
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
    patch = request.get_json(force=True) or {}
    resource = RESOURCES[resource_id]
    apply_merge_patch(resource, patch)
    return jsonify({"status": "patched", "resource": resource})

if __name__ == "__main__":
    app.run(port=5000)

import functools
from flask import request, jsonify
from models import app, ACCOUNTS
from serializers import deserialize_plan_update

def _require_auth(f):
    @functools.wraps(f)
    def decorated(account_id, *args, **kwargs):
        if request.headers.get("X-Account-Id") != account_id:
            return jsonify({"error": "Forbidden"}), 403
        return f(account_id, *args, **kwargs)
    return decorated

@app.route("/accounts/<account_id>/plan", methods=["PUT"])
@_require_auth
def update_plan(account_id):
    if account_id not in ACCOUNTS:
        return jsonify({"error": "Not found"}), 404
    data = request.get_json(force=True) or {}
    updates = deserialize_plan_update(data)
    ACCOUNTS[account_id].update(updates)
    return jsonify({"status": "updated", "tier": ACCOUNTS[account_id]["tier"]})

if __name__ == "__main__":
    app.run(port=5000)

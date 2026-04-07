from flask import Flask, jsonify, request, session
from auth import login_required, get_current_user_id
from models import get_customers_page, get_tenant_for_user

app = Flask(__name__)
app.secret_key = "dev-secret-replace-in-prod"

@app.route("/api/session/login", methods=["POST"])
def login():
    data = request.get_json(force=True) or {}
    user_id = data.get("user_id")
    if not user_id:
        return jsonify({"error": "user_id required"}), 400
    session["user_id"] = int(user_id)
    return jsonify({"ok": True})

@app.route("/api/customers")
@login_required
def list_customers():
    after = int(request.args.get("after", 0))
    uid = get_current_user_id()
    tenant_id = get_tenant_for_user(uid)
    if tenant_id is None:
        return jsonify({"error": "Tenant not found"}), 404

# vuln-code-snippet start ChainScenario0090B
    customers = get_customers_page(tenant_id, after)  # vuln-code-snippet target-line ChainScenario0090B
# vuln-code-snippet end ChainScenario0090B

    next_cursor = customers[-1]["id"] if customers else None
    return jsonify({"customers": customers, "next_cursor": next_cursor})

if __name__ == "__main__":
    app.run(port=5000)

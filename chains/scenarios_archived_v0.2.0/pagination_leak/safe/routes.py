"""Flask routes for the customer-list pagination API -- SAFE variant.

The pagination query always includes the caller's tenant_id so the
cursor value cannot be used to cross tenant boundaries.

Chain: broken -- tenant_id included in every paginated query
CWE-200: Exposure of Sensitive Information (remediated)
"""
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
    """Return a page of customers.

    FIXED: tenant_id is resolved from the session and included in
    every query.  A crafted cursor cannot produce records from another
    tenant because the WHERE clause always filters by tenant_id.
    """
    after = int(request.args.get("after", 0))
    uid = get_current_user_id()
    tenant_id = get_tenant_for_user(uid)
    if tenant_id is None:
        return jsonify({"error": "Tenant not found"}), 404

# vuln-code-snippet start chain_pagination_leak_safe
    customers = get_customers_page(tenant_id, after)  # vuln-code-snippet safe-line chain_pagination_leak_safe
# vuln-code-snippet end chain_pagination_leak_safe

    next_cursor = customers[-1]["id"] if customers else None
    return jsonify({"customers": customers, "next_cursor": next_cursor})


if __name__ == "__main__":
    app.run(port=5000)

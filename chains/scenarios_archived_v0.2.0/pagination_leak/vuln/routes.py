"""Flask routes for the customer-list pagination API -- VULNERABLE variant.

GET /api/customers accepts an 'after' cursor (plain integer row id).
The route passes the cursor directly to a query that omits the
tenant_id filter, so an attacker can supply a cross-tenant cursor to
read another tenant's customer records.

Chain: authenticated session + crafted cursor value -> records from
  other tenants
Individual findings: cursor not scoped to tenant (medium)
Chain finding: cross-tenant customer data exfiltration via pagination (high)
CWE-200: Exposure of Sensitive Information
"""
from flask import Flask, jsonify, request, session
from auth import login_required, get_current_user_id
from models import get_customers_page_no_tenant_check, get_tenant_for_user

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

    VULNERABLE: the after cursor is passed without re-scoping the
    query to the caller's tenant.  Guessing row ids from another
    tenant returns their customer records.
    """
    after = int(request.args.get("after", 0))

# vuln-code-snippet start chain_pagination_leak_vuln
    customers = get_customers_page_no_tenant_check(after)  # vuln-code-snippet vuln-line chain_pagination_leak_vuln
# vuln-code-snippet end chain_pagination_leak_vuln

    next_cursor = customers[-1]["id"] if customers else None
    return jsonify({"customers": customers, "next_cursor": next_cursor})


if __name__ == "__main__":
    app.run(port=5000)

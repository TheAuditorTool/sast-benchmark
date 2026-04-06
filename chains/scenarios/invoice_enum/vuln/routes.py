"""Flask routes for the billing invoice API -- VULNERABLE variant.

GET /api/invoices/<invoice_id> returns a full invoice record
including amount, line items, and due date without verifying that the
authenticated user belongs to the tenant that owns the invoice.
Sequential invoice IDs make enumeration trivial.

Chain: authenticated session + sequential invoice_id -> cross-tenant
  billing data
Individual findings: missing tenant check on invoice lookup (medium)
Chain finding: cross-tenant financial data exfiltration (high)
CWE-862: Missing Authorization
"""
from flask import Flask, jsonify, session
from auth import login_required, get_current_user_id
from models import get_invoice, list_tenant_invoices, get_tenant_for_user

app = Flask(__name__)
app.secret_key = "dev-secret-replace-in-prod"


@app.route("/api/session/login", methods=["POST"])
def login():
    from flask import request
    data = request.get_json(force=True) or {}
    user_id = data.get("user_id")
    if not user_id:
        return jsonify({"error": "user_id required"}), 400
    session["user_id"] = int(user_id)
    return jsonify({"ok": True})


@app.route("/api/invoices")
@login_required
def list_invoices():
    """Return invoices for the caller's tenant."""
    uid = get_current_user_id()
    tenant_id = get_tenant_for_user(uid)
    if tenant_id is None:
        return jsonify({"error": "Tenant not found"}), 404
    return jsonify(list_tenant_invoices(tenant_id))


@app.route("/api/invoices/<int:invoice_id>")
@login_required
def get_invoice_detail(invoice_id):
    """Return a single invoice.

    VULNERABLE: no tenant check -- an attacker iterates invoice_id to
    harvest billing records from all tenants.
    """
# vuln-code-snippet start chain_invoice_enum_vuln
    invoice = get_invoice(invoice_id)  # vuln-code-snippet vuln-line chain_invoice_enum_vuln
# vuln-code-snippet end chain_invoice_enum_vuln
    if invoice is None:
        return jsonify({"error": "Invoice not found"}), 404
    return jsonify(invoice)


if __name__ == "__main__":
    app.run(port=5000)

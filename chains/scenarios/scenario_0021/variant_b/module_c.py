from flask import Flask, jsonify, session
from module_a import login_required, get_current_user_id
from module_b import get_invoice, list_tenant_invoices, get_tenant_for_user

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
    uid = get_current_user_id()
    tenant_id = get_tenant_for_user(uid)
    if tenant_id is None:
        return jsonify({"error": "Tenant not found"}), 404
    return jsonify(list_tenant_invoices(tenant_id))

@app.route("/api/invoices/<int:invoice_id>")
@login_required
def get_invoice_detail(invoice_id):
    invoice = get_invoice(invoice_id)
    if invoice is None:
        return jsonify({"error": "Invoice not found"}), 404
    uid = get_current_user_id()
    caller_tenant = get_tenant_for_user(uid)
    if invoice["tenant_id"] != caller_tenant:
        return jsonify({"error": "Access denied"}), 403
    return jsonify(invoice)

if __name__ == "__main__":
    app.run(port=5000)

from flask import jsonify
from routes import app, search_transactions, account_summary
from auth import require_auth, login, complete_mfa

@app.route("/api/health")
def health():
    return jsonify({"status": "ok"})

app.add_url_rule("/auth/login", view_func=login, methods=["POST"])
app.add_url_rule("/auth/mfa", view_func=complete_mfa, methods=["POST"])
app.add_url_rule("/finance/transactions", view_func=require_auth(search_transactions))
app.add_url_rule("/finance/account", view_func=require_auth(account_summary))

if __name__ == "__main__":
    app.run(port=5005)

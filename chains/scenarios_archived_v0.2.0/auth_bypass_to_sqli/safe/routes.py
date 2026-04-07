"""Route configuration -- SAFE variant.

The admin search endpoint is registered WITH the auth middleware.
The chain is broken: even though the SQL injection exists in app.py,
an unauthenticated user cannot reach it.

Chain: unauthenticated request -> /admin/search (auth REQUIRED) -> blocked
Individual findings: SQLi (medium) -- still present but behind auth
Chain finding: none -- chain is not exploitable
"""
from app import app, admin_search_users, admin_dashboard
from middleware import require_auth


# Public routes -- no auth required
@app.route("/api/health")
def health():
    return {"status": "ok"}


# Admin routes -- auth correctly applied to ALL admin endpoints
app.add_url_rule("/admin/dashboard", view_func=require_auth(admin_dashboard))
app.add_url_rule("/admin/search", view_func=require_auth(admin_search_users))  # FIXED: auth applied


if __name__ == "__main__":
    app.run(port=5000)

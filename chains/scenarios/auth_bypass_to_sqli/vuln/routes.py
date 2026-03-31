"""Route configuration -- VULNERABLE variant.

The admin search endpoint is registered WITHOUT the auth middleware.
This is the chain enabler: an unauthenticated user can reach the
SQL injection in app.admin_search_users().

Chain: unauthenticated request -> /admin/search (no auth) -> SQLi
Individual findings: missing auth (medium) + SQLi (medium)
Chain finding: unauthenticated SQLi (critical)
"""
from app import app, admin_search_users, admin_dashboard
from middleware import require_auth


# Public routes -- no auth required
@app.route("/api/health")
def health():
    return {"status": "ok"}


# Admin routes -- auth SHOULD be required but is MISSING on /admin/search
app.add_url_rule("/admin/dashboard", view_func=require_auth(admin_dashboard))
app.add_url_rule("/admin/search", view_func=admin_search_users)  # BUG: no auth


if __name__ == "__main__":
    app.run(port=5000)

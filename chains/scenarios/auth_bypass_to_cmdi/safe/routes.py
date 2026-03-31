"""Route configuration for diagnostics app."""
from app import app, run_diagnostics
from middleware import require_auth

app.add_url_rule("/api/diagnostics", view_func=require_auth(run_diagnostics))

if __name__ == "__main__":
    app.run(port=5001)

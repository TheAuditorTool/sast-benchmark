from module_a import app, run_diagnostics
from module_b import require_auth

app.add_url_rule("/api/diagnostics", view_func=require_auth(run_diagnostics))

if __name__ == "__main__":
    app.run(port=5001)

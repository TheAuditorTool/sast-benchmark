from urllib.parse import urlparse
from flask import request, Response
from app import app
from redirect_utils import build_redirect_page

ALLOWED_HOST = "app.example.com"

def _safe_fallback(url):
    parsed = urlparse(url)
    if parsed.netloc and parsed.netloc != ALLOWED_HOST:
        return "/dashboard"
    return url

@app.route("/redirect")
def js_redirect():
    fallback = _safe_fallback(request.args.get("fallback", "/dashboard"))

# vuln-code-snippet start ChainScenario0038A
    html = build_redirect_page(fallback)
    return Response(html, mimetype="text/html")  # vuln-code-snippet target-line ChainScenario0038A
# vuln-code-snippet end ChainScenario0038A

if __name__ == "__main__":
    app.run(port=5000)

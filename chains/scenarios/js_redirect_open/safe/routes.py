"""Route definitions for JS fragment open redirect scenario -- SAFE variant.

GET /redirect validates the ?fallback= parameter before embedding it.
External fallback URLs are replaced with /dashboard.

Chain: /redirect?fallback=https://evil.com -> fallback rejected -> /dashboard embedded safely
Individual findings: none
Chain finding: none -- fallback is validated server-side
"""
from urllib.parse import urlparse
from flask import request, Response
from app import app
from redirect_utils import build_redirect_page

ALLOWED_HOST = "app.example.com"


def _safe_fallback(url):
    """Return url if it is same-host or relative, otherwise /dashboard."""
    parsed = urlparse(url)
    if parsed.netloc and parsed.netloc != ALLOWED_HOST:
        return "/dashboard"
    return url


@app.route("/redirect")
def js_redirect():
    """Serve a redirect page with a validated fallback URL."""
    fallback = _safe_fallback(request.args.get("fallback", "/dashboard"))

# vuln-code-snippet start chain_js_redirect_safe
    html = build_redirect_page(fallback)
    return Response(html, mimetype="text/html")  # vuln-code-snippet safe-line chain_js_redirect_safe
# vuln-code-snippet end chain_js_redirect_safe


if __name__ == "__main__":
    app.run(port=5000)

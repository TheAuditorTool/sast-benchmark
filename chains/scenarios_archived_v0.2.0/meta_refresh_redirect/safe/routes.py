"""Route definitions for meta-refresh open redirect scenario -- SAFE variant.

GET /loading validates the ?next= URL before embedding it in the meta refresh tag.
External destinations are replaced with /dashboard.

Chain: GET /loading?next=https://evil.com -> host check rejects -> meta refresh to /dashboard
Individual findings: none
Chain finding: none -- URL is validated before being embedded
"""
from urllib.parse import urlparse
from flask import request, Response
from app import app
from redirect_utils import build_meta_refresh_page

ALLOWED_HOST = "app.example.com"


def _safe_next(url):
    """Return url if same-host or relative, else /dashboard."""
    parsed = urlparse(url)
    if parsed.netloc and parsed.netloc != ALLOWED_HOST:
        return "/dashboard"
    return url


@app.route("/loading")
def loading():
    """Show a loading page that redirects via validated meta refresh."""
    next_url = _safe_next(request.args.get("next", "/dashboard"))

# vuln-code-snippet start chain_meta_refresh_safe
    html = build_meta_refresh_page(next_url)
    return Response(html, mimetype="text/html")  # vuln-code-snippet safe-line chain_meta_refresh_safe
# vuln-code-snippet end chain_meta_refresh_safe


if __name__ == "__main__":
    app.run(port=5000)

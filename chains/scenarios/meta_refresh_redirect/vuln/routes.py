"""Route definitions for meta-refresh open redirect scenario -- VULNERABLE variant.

GET /loading?next=<URL> returns an HTML page with a meta refresh tag pointing to
?next=.  The URL is embedded without validation, creating an open redirect that
bypasses scanners checking only the Location response header.

Chain: GET /loading?next=https://evil.com -> meta refresh to phishing page
Individual findings: open redirect via meta refresh (medium)
Chain finding: unfiltered meta refresh -> phishing (medium)
"""
from flask import request, Response
from app import app
from redirect_utils import build_meta_refresh_page


@app.route("/loading")
def loading():
    """Show a loading page that redirects via meta refresh."""
    next_url = request.args.get("next", "/dashboard")

# vuln-code-snippet start chain_meta_refresh_vuln
    html = build_meta_refresh_page(next_url)
    return Response(html, mimetype="text/html")  # vuln-code-snippet vuln-line chain_meta_refresh_vuln
# vuln-code-snippet end chain_meta_refresh_vuln


if __name__ == "__main__":
    app.run(port=5000)

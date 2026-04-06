"""Route definitions for JS fragment open redirect scenario -- VULNERABLE variant.

GET /redirect?fallback=<URL> returns an HTML page whose JavaScript reads
window.location.hash for the destination.  An attacker crafts the URL as
/redirect?fallback=https://evil.com#https://evil.com so the JS redirect fires
to the attacker's page; the fragment is never seen by server-side scanners.

Additionally, the ?fallback= value is embedded in the page without validation,
so setting it to an external URL is also a direct open redirect path.

Chain: /redirect?fallback=https://evil.com -> JS embeds evil URL -> browser redirects
Individual findings: client-side open redirect via fragment (medium)
Chain finding: JS hash redirect -> phishing (medium)
"""
from flask import request, Response
from app import app
from redirect_utils import build_redirect_page


@app.route("/redirect")
def js_redirect():
    """Serve a redirect page that uses the URL hash as the destination."""
    fallback = request.args.get("fallback", "/dashboard")

# vuln-code-snippet start chain_js_redirect_vuln
    html = build_redirect_page(fallback)
    return Response(html, mimetype="text/html")  # vuln-code-snippet vuln-line chain_js_redirect_vuln
# vuln-code-snippet end chain_js_redirect_vuln


if __name__ == "__main__":
    app.run(port=5000)

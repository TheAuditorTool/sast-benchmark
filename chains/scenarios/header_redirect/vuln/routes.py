"""Route definitions for Host-header open redirect scenario -- VULNERABLE variant.

GET /go?to=<path> builds an absolute URL from the Host header and the supplied
path, then redirects.  By forging a Host header an attacker routes the user
to their domain while the URL still looks like a relative path.

Chain: GET /go?to=/dashboard with Host: evil.com -> redirect to https://evil.com/dashboard
Individual findings: Host header injection (medium)
Chain finding: Host header -> open redirect -> phishing (high)
"""
from flask import request, redirect
from app import app
from redirect_utils import build_redirect_url


@app.route("/go")
def go():
    """Redirect to an absolute URL built from Host header + path."""
    path = request.args.get("to", "/")
    if not path.startswith("/"):
        path = "/"

# vuln-code-snippet start chain_header_redirect_vuln
    url = build_redirect_url(path)
    return redirect(url)  # vuln-code-snippet vuln-line chain_header_redirect_vuln
# vuln-code-snippet end chain_header_redirect_vuln


if __name__ == "__main__":
    app.run(port=5000)

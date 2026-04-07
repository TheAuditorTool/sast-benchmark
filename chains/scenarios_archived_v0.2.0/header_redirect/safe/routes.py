"""Route definitions for Host-header open redirect scenario -- SAFE variant.

GET /go?to=<path> builds the redirect URL from a hardcoded base, ignoring the Host header.

Chain: GET /go?to=/dashboard with Host: evil.com -> redirect to https://app.example.com/dashboard
Individual findings: none
Chain finding: none -- Host header is not used in redirect construction
"""
from flask import request, redirect
from app import app
from redirect_utils import build_redirect_url


@app.route("/go")
def go():
    """Redirect to an absolute URL built from the hardcoded base + path."""
    path = request.args.get("to", "/")
    if not path.startswith("/"):
        path = "/"

# vuln-code-snippet start chain_header_redirect_safe
    url = build_redirect_url(path)
    return redirect(url)  # vuln-code-snippet safe-line chain_header_redirect_safe
# vuln-code-snippet end chain_header_redirect_safe


if __name__ == "__main__":
    app.run(port=5000)

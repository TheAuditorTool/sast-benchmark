"""Route definitions for subdomain-regex open redirect scenario -- VULNERABLE variant.

GET /go?url=<URL> validates the URL against a subdomain regex, then redirects.
The regex .*\.example\.com also matches evil-example.com, so an attacker
can bypass the check with a domain that ends in example.com.

Chain: /go?url=https://evil-example.com -> regex matches (bypass) -> redirect to attacker site
Individual findings: regex subdomain check bypass (medium)
Chain finding: regex bypass -> open redirect -> phishing (high)
"""
from flask import request, redirect, jsonify
from app import app
from redirect_utils import is_safe_redirect_url


@app.route("/go")
def go():
    """Validate and follow a redirect URL."""
    url = request.args.get("url", "")
    if not url:
        return jsonify({"error": "url required"}), 400

# vuln-code-snippet start chain_subdomain_redirect_vuln
    if not is_safe_redirect_url(url):
        return jsonify({"error": "Redirect not allowed"}), 400
    return redirect(url)  # vuln-code-snippet vuln-line chain_subdomain_redirect_vuln
# vuln-code-snippet end chain_subdomain_redirect_vuln


if __name__ == "__main__":
    app.run(port=5000)

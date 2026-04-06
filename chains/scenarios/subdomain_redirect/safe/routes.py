"""Route definitions for subdomain-regex open redirect scenario -- SAFE variant.

GET /go?url=<URL> uses urlparse-based host validation that correctly rejects
evil-example.com while allowing sub.example.com.

Chain: /go?url=https://evil-example.com -> netloc check fails -> 400 blocked
Individual findings: none
Chain finding: none -- host validation uses urlparse instead of regex
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

# vuln-code-snippet start chain_subdomain_redirect_safe
    if not is_safe_redirect_url(url):
        return jsonify({"error": "Redirect not allowed"}), 400
    return redirect(url)  # vuln-code-snippet safe-line chain_subdomain_redirect_safe
# vuln-code-snippet end chain_subdomain_redirect_safe


if __name__ == "__main__":
    app.run(port=5000)
